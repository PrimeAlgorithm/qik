//! Handles formatting for requests and responses.
//!
//! This module handles converting a [`RequestSpec`] and [`ResponseData`]
//! into human-readable strings with colors.

use crate::{
    handlers::http::requests::set_payload::Payload,
    models::http::{RequestSpec, ResponseData, Transaction},
    output::format::{
        body::format_body_bytes, format_headers::format_headers, payload::format_payload,
        status_color::status_code_color,
    },
};
use anyhow::anyhow;
use owo_colors::OwoColorize;

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

    if let Payload::Body(body_info) = &request.body {
        let content_length = body_info.content.len();
        out.push_str(&format!(
            "\n{}: {}",
            "content-length".bright_black(),
            content_length
        ));
    }

    let headers_formatted = format_headers(&request.headers)?;
    out.push_str(&headers_formatted);

    let formatted_payload = format_payload(&request.body, &request.headers)?;

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

    let formatted_payload = format_body_bytes(&response.body, &response.headers)?;

    if let Some(payload) = formatted_payload {
        out.push_str(&format!("\n\n{payload}"));
    } else {
        out.push_str("\n\n<no body>");
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

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
