use crate::output::format::{
    detect::{is_json_like, is_xml_like},
    pretty::pretty_json,
};
use bytes::Bytes;
use mime::Mime;
use reqwest::header::{CONTENT_TYPE, HeaderMap};

// Return body of request/response formatted if possible.
pub fn format_body_bytes(
    body: &Bytes,
    headers: &HeaderMap,
) -> Result<Option<String>, anyhow::Error> {
    let formatted_payload;
    let bytes_to_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => return Ok(Some(format!("<non-UTF8 body ({} bytes)>", body.len()))),
    };

    let content_type: Option<Mime> = headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<Mime>().ok());

    if let Some(mime_type) = content_type {
        if is_json_like(&mime_type) {
            formatted_payload = Some(pretty_json(bytes_to_str)?);
        } else if is_xml_like(&mime_type) {
            formatted_payload = Some(bytes_to_str.to_string());
        } else {
            formatted_payload = Some(bytes_to_str.to_string());
        }
    } else {
        formatted_payload = Some(bytes_to_str.to_string());
    }

    Ok(formatted_payload)
}
