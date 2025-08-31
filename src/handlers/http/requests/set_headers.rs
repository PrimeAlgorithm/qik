use crate::handlers::http::requests::request_info::RequestInformation;
use reqwest::header::HeaderMap;

/// Puts header arguments into a [`HeaderMap`].
pub fn set_headers(req_info: &RequestInformation) -> Result<HeaderMap, anyhow::Error> {
    let mut request_headers = HeaderMap::new();

    if let Some(headers) = &req_info.common.header {
        for (key, value) in headers {
            request_headers.append(key, value.clone());
        }
    }

    Ok(request_headers)
}
