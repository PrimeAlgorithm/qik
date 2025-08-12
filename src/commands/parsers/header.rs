use reqwest::header::{HeaderName, HeaderValue};

pub fn parse_header(header: &str) -> Result<(HeaderName, HeaderValue), String> {
    let (header_key_unformatted, header_value_formatted) = header
        .split_once(':')
        .ok_or_else(|| "Headers must be in `Key: Value` form")?;

    let key = HeaderName::from_bytes(header_key_unformatted.as_bytes())
        .map_err(|_| "Invalid header name".to_owned())?;
    let value = HeaderValue::from_bytes(header_value_formatted.trim().as_bytes())
        .map_err(|_| "invalid header value".to_owned())?;

    Ok((key, value))
}
