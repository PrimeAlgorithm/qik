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
    if body.is_empty() {
        return Ok(None);
    }

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
            // Servers occasionally label error pages or truncated responses as
            // JSON. Rendering the original body is more useful than failing the
            // entire command in that situation.
            formatted_payload =
                Some(pretty_json(bytes_to_str).unwrap_or_else(|_| bytes_to_str.to_owned()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn empty_body_is_absent() {
        assert_eq!(
            format_body_bytes(&Bytes::new(), &HeaderMap::new()).unwrap(),
            None
        );
    }

    #[test]
    fn malformed_json_falls_back_to_original_body() {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        assert_eq!(
            format_body_bytes(&Bytes::from_static(b"not json"), &headers).unwrap(),
            Some("not json".to_owned())
        );
    }
}
