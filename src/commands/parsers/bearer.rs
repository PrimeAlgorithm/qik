use crate::util::strip_quotes::strip_matching_quotes;

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
