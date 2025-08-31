use crate::handlers::http::requests::request_info::RequestInformation;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};

/// Adds cookies to an already existing [`HeaderMap`] and returns it.
pub fn set_cookies(
    req_info: &RequestInformation,
    mut request_headers: HeaderMap,
) -> Result<HeaderMap, anyhow::Error> {
    if !request_headers.contains_key(COOKIE) {
        if let Some(cookies) = &req_info.common.cookie {
            if !cookies.is_empty() {
                let mut cookie_str = String::new();
                for (i, (name, value)) in cookies.iter().enumerate() {
                    if i > 0 {
                        cookie_str.push_str("; ");
                    }
                    cookie_str.push_str(name);
                    cookie_str.push('=');
                    cookie_str.push_str(value);
                }

                request_headers.append(COOKIE, HeaderValue::from_str(&cookie_str)?);
            }
        }
    }

    Ok(request_headers)
}
