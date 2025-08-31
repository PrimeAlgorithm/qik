use base64::prelude::*;
use reqwest::header::{AUTHORIZATION, HeaderName, HeaderValue};

use crate::handlers::http::requests::request_info::RequestInformation;

/// Creates authorization header from req_info, if possible.
pub fn load_auth(
    req_info: &RequestInformation,
) -> Result<Option<(HeaderName, HeaderValue)>, anyhow::Error> {
    if let Some(user_credentials) = &req_info.common.auth {
        let credentials_encoded = BASE64_STANDARD.encode(user_credentials);
        let formatted_auth_header = format!("Basic {}", credentials_encoded);

        return Ok(Some((
            AUTHORIZATION,
            HeaderValue::from_str(&formatted_auth_header)?,
        )));
    }

    if let Some(bearer_token) = &req_info.common.bearer {
        let formatted_bearer_token = format!("Bearer {}", bearer_token);

        return Ok(Some((
            AUTHORIZATION,
            HeaderValue::from_str(&formatted_bearer_token)?,
        )));
    }

    Ok(None)
}
