use reqwest::header::{HeaderName, HeaderValue};

/// Parses a `Key: Value` string into a header pair.
///
/// # Errors
/// Returns an error if no `:` is present, or if the key or value
/// is not a valid HTTP header.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_header() {
        let (header_name, header_value) = parse_header("Key: Value").unwrap();
        assert_eq!(header_name.as_str(), "key");
        assert_eq!(header_value.to_str().unwrap(), "Value");
    }

    #[test]
    fn test_header_no_whitespace() {
        let (header_name, header_value) = parse_header("Key:Value").unwrap();
        assert_eq!(header_name.as_str(), "key");
        assert_eq!(header_value.to_str().unwrap(), "Value");
    }

    #[test]
    fn test_missing_colon() {
        let err = parse_header("keyvalue").unwrap_err();
        assert_eq!(err, "Headers must be in `Key: Value` form");
    }
}
