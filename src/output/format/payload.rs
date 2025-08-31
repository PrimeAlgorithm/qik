use crate::{
    handlers::http::requests::set_payload::Payload, output::format::body::format_body_bytes,
};
use bytes::Bytes;
use reqwest::header::HeaderMap;

/// Formats request payload.
pub fn format_payload(
    payload: &Payload,
    headers: &HeaderMap,
) -> Result<Option<String>, anyhow::Error> {
    match &payload {
        Payload::Body(data) => format_body_bytes(&data.content, headers),
        Payload::Form(map) => {
            let url_encoded = serde_urlencoded::to_string(map)?;
            format_body_bytes(&Bytes::from(url_encoded), &headers)
        }
        Payload::Multipart(_multipart) => {
            Ok(Some(String::from("<multipart/form-data body (streamed)>")))
        }
        Payload::None => Ok(None),
    }
}
