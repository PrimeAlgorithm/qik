//! Handles formatting for requests and responses.
//!
//! This module handles converting a [`RequestSpec`] and [`ResponseData`]
//! into human-readable strings with colors.

use crate::models::http::{RequestSpec, ResponseData, Transaction};
use anyhow::anyhow;
use bytes::Bytes;
use owo_colors::OwoColorize;
use reqwest::{
    StatusCode,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, PROXY_AUTHORIZATION},
};

/// Turn a `(RequestSpec, ResponseData)` into two terminal-friendly strings.
pub fn format_transaction(transaction: Transaction) -> Result<(String, String), anyhow::Error> {
    let request_formatted = format_request(transaction.0)?;
    let response_formatted = format_response(transaction.1)?;

    Ok((request_formatted, response_formatted))
}

fn format_request(request: RequestSpec) -> Result<String, anyhow::Error> {
    let mut out = String::new();

    let title = "Request:".bold();
    out.push_str(&format!("{title}"));

    let url = request.url;

    let version_display = match request.version {
        Some(requested) if requested != request.negotiated => {
            format!("{:?} (negotiated: {:?})", requested, request.negotiated)
        }
        Some(requested) => format!("{:?}", requested),
        None => format!("{:?} (auto-negotiated)", request.negotiated),
    };

    let method = request.method.as_str();
    let cyan_method_display = method.cyan();
    let stylized_method_display = cyan_method_display.bold();

    out.push_str(&format!(
        "\n{stylized_method_display} {url} {version_display}"
    ));

    let host = url
        .host()
        .map(|u| u.to_string())
        .ok_or_else(|| anyhow!("URL has no host"))?;
    out.push_str(&format!("\n{}: {}", "host".bright_black(), host));

    if let Some(body) = &request.body {
        let content_length = body.len().to_string();
        out.push_str(&format!(
            "\n{}: {}",
            "content-length".bright_black(),
            content_length
        ));
    }

    let headers_formatted = format_headers(&request.headers)?;
    out.push_str(&headers_formatted);

    let formatted_payload = format_body(&request.body, &request.headers)?;

    if let Some(payload) = formatted_payload {
        out.push_str(&format!("\n\n{payload}"));
    } else {
        out.push_str("\n\n<no body>");
    }

    Ok(out)
}

fn format_response(response: ResponseData) -> Result<String, anyhow::Error> {
    let mut out = String::new();

    let title = "Response:".bold();
    out.push_str(&format!("{title}"));

    let version = format!("{:?}", response.version);
    let response_code = status_code_color(response.status);
    out.push_str(&format!("\n{version} {response_code}"));

    let headers_formatted = format_headers(&response.headers)?;
    out.push_str(&headers_formatted);

    let formatted_payload = format_body(&Some(response.body), &response.headers)?;

    if let Some(payload) = formatted_payload {
        out.push_str(&format!("\n\n{payload}"));
    } else {
        out.push_str("\n\n<no body>");
    }

    Ok(out)
}

// helpers

// Return body of request/response formatted if possible.
fn format_body(body: &Option<Bytes>, headers: &HeaderMap) -> Result<Option<String>, anyhow::Error> {
    let mut formatted_payload = None;
    let content_type_option = headers.get(CONTENT_TYPE);

    if let Some(payload) = body {
        let bytes_to_str = std::str::from_utf8(&payload)?;

        // If the user has specified that content_type is json or xml,
        // we can pretty print it.
        if let Some(content_type) = content_type_option {
            match content_type.to_str()? {
                "application/json" => {
                    formatted_payload = Some(get_pretty_json(bytes_to_str)?);
                }
                "application/xml" => {}
                _ => {
                    // Some other format that cannot be pretty printed.
                    formatted_payload = Some(bytes_to_str.to_string());
                }
            }
        } else {
            // Content type was not specified, so just place string.
            formatted_payload = Some(bytes_to_str.to_string());
        }
    }

    Ok(formatted_payload)
}

/// Returns pretty-print JSON bodies.
fn get_pretty_json(json_str: &str) -> Result<String, anyhow::Error> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;
    let formatted_json = serde_json::to_string_pretty(&json_value)?;

    Ok(formatted_json)
}

fn status_code_color(code: StatusCode) -> String {
    let styled = match code.as_u16() {
        100..=199 => format!("{}", code.as_str().cyan().bold()),
        200..=299 => format!("{}", code.as_str().green().bold()),
        300..=399 => format!("{}", code.as_str().yellow().bold()),
        400..=499 => format!("{}", code.as_str().red().bold()),
        _ => format!("{}", code.as_str().bright_red().bold()),
    };

    styled
}

fn is_sensitive(name: &HeaderName) -> bool {
    name == AUTHORIZATION
        || name == PROXY_AUTHORIZATION
        || name.as_str().eq_ignore_ascii_case("cookie")
}

/// Return header block (`\nKey: Value…`) or empty string.
fn format_headers(headers: &HeaderMap) -> Result<String, anyhow::Error> {
    let mut out = String::new();

    for (name, value) in headers {
        let key = name.bright_black();

        if name == AUTHORIZATION {
            let s = value.to_str().unwrap_or_default();
            let scheme = s.splitn(2, char::is_whitespace).next().unwrap_or("");
            if scheme.is_empty() {
                out.push_str(&format!("\n{key}: <redacted>"));
            } else {
                out.push_str(&format!("\n{key}: {scheme} <redacted>"));
            }
            continue;
        }

        if is_sensitive(name) {
            out.push_str(&format!("\n{key}: <redacted>"));
            continue;
        }

        match value.to_str() {
            Ok(v) => out.push_str(&format!("\n{key}: {v}")),
            Err(_) => out.push_str(&format!(
                "\n{key}: <non-UTF8 value ({} bytes)>",
                value.as_bytes().len()
            )),
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderValue;

    use super::*;

    #[test]
    fn test_basic_header_redaction() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic user:pass"));
        let formatted_header = format_headers(&headers).unwrap();
        assert_eq!(
            formatted_header,
            format!("\n{}: Basic <redacted>", "authorization".bright_black())
        );
    }

    #[test]
    fn test_bearer_header_redaction() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer user:pass"));
        let formatted_header = format_headers(&headers).unwrap();
        assert_eq!(
            formatted_header,
            format!("\n{}: Bearer <redacted>", "authorization".bright_black())
        );
    }
}
