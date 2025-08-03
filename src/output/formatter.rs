use crate::models::http::{RequestSpec, ResponseData, Transaction};
use bytes::Bytes;
use owo_colors::OwoColorize;
use reqwest::{
    StatusCode,
    header::{CONTENT_TYPE, HeaderMap},
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
    let version = format!("{:?}", request.version);
    let method = request.method.as_str();
    let cyan_method_display = method.cyan();
    let stylized_method_display = cyan_method_display.bold();
    out.push_str(&format!("\n{stylized_method_display} {url} {version}"));

    let host = url.host().unwrap().to_string();
    out.push_str(&format!("\n{}: {}", "host".bright_black(), host));

    if let Some(body) = &request.body {
        let content_length = body.len().to_string();
        out.push_str(&format!(
            "\n{}: {}",
            "content-length:".bright_black(),
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
        let bytes_to_str = std::str::from_utf8(&payload).unwrap();

        // If the user has specified that content_type is json or xml,
        // we can pretty print it.
        if let Some(content_type) = content_type_option {
            match content_type.to_str().unwrap() {
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

/// Return header block (`\nKey: Value…`) or empty string.
fn format_headers(headers: &HeaderMap) -> Result<String, anyhow::Error> {
    let mut out = String::new();

    for (header_name, header_value) in headers {
        let header_key_unwrapped = header_name;
        let header_key_stylized = header_key_unwrapped.bright_black();
        let header_value_str = header_value.to_str()?;

        out.push_str(&format!("\n{header_key_stylized}: {header_value_str}"));
    }

    Ok(out)
}
