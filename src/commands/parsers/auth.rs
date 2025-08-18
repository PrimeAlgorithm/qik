pub fn parse_auth(user_credentials: &str) -> Result<String, String> {
    if !user_credentials.contains(':') {
        return Err("Credentials must be in user:pass form.".into());
    }

    Ok(user_credentials.to_owned())
}
