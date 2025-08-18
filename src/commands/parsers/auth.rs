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
