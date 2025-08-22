/// Parses and validates basic authentication credentials.
///
/// Ensures the input is in `user:pass` form without further validation.
///
/// # Errors
/// Returns an error if the string does not contain a `:`.
pub fn parse_auth(user_credentials: &str) -> Result<String, String> {
    if !user_credentials.contains(':') {
        return Err("Credentials must be in user:pass form.".into());
    }

    Ok(user_credentials.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_user_pass() {
        let err = parse_auth("userpass").unwrap_err();
        assert_eq!(err, "Credentials must be in user:pass form.");
    }

    #[test]
    fn test_missing_colon() {
        assert!(parse_auth("userpass").is_err());
    }

    #[test]
    fn test_leading_white_space() {
        assert_eq!(parse_auth(" user:pass ").unwrap(), " user:pass ");
    }

    #[test]
    fn test_null_user_and_pass() {
        assert_eq!(parse_auth(":pass").unwrap(), ":pass");
        assert_eq!(parse_auth("user:").unwrap(), "user:");
    }

    #[test]
    fn test_multiple_colons() {
        assert_eq!(parse_auth("u:s:e:r:p:a:s:s").unwrap(), "u:s:e:r:p:a:s:s");
    }
}
