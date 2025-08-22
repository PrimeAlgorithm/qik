use crate::util::strip_quotes::strip_matching_quotes;

/// Parses and validates a bearer token.
///
/// Removes surrounding whitespace and matching quotes if possible.
///
/// # Errors
/// Returns an error if the token is empty after trimming, or if
/// it contains whitespace/control characters.
pub fn parse_bearer(token: &str) -> Result<String, String> {
    let formatted_token = strip_matching_quotes(token.trim());

    if formatted_token.is_empty() {
        return Err("Bearer token must not be empty after being trimmed".to_owned());
    }

    if formatted_token.chars().any(char::is_whitespace) {
        return Err("Whitespace is not allowed in bearer token".to_owned());
    }

    if formatted_token.chars().any(char::is_control) {
        return Err("Control characters are not allowed in bearer token".to_owned());
    }

    Ok(formatted_token.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_token() {
        assert_eq!(parse_bearer("access-token").unwrap(), "access-token");
    }

    #[test]
    fn test_empty_token_err() {
        let err = parse_bearer("").unwrap_err();
        assert_eq!(err, "Bearer token must not be empty after being trimmed");
    }

    #[test]
    fn test_whitespace_err() {
        let err = parse_bearer("access token").unwrap_err();
        assert_eq!(err, "Whitespace is not allowed in bearer token");
    }

    #[test]
    fn test_control_char_err() {
        let err = parse_bearer("access\0token").unwrap_err();
        assert_eq!(err, "Control characters are not allowed in bearer token");
    }
}
