/// Parses a `name=value` cookie string.
///
/// Splits the input on the first `=` into `(key, value)`.
///
/// # Errors
/// Returns an error if no `=` is present, if the name is empty, or if
/// invalid characters are detected.
pub fn parse_cookie(input: &str) -> Result<(String, String), String> {
    let (name_raw, value) = input
        .split_once('=')
        .ok_or_else(|| "Cookie must be in `name=value` form")?;

    let name = name_raw.trim();

    if name.is_empty() {
        return Err("Cookie name may not be empty (`=value` is invalid)".to_owned());
    }

    if name.chars().any(|c| c == ';' || c.is_control()) {
        return Err("Cookie name may not contain semicolons or control characters".to_owned());
    }

    if value.chars().any(|c| c == ';' || c.is_control()) {
        return Err("Cookie value may not contain semicolons or control characters".to_owned());
    }

    Ok((name.to_owned(), value.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cookie() {
        assert_eq!(
            parse_cookie("key=value").unwrap(),
            ("key".to_owned(), "value".to_owned())
        );
    }

    #[test]
    fn test_empty_value_ok() {
        assert_eq!(
            parse_cookie("session=").unwrap(),
            ("session".to_owned(), "".to_owned())
        );
    }

    #[test]
    fn test_missing_equals() {
        let err = parse_cookie("keyvalue").unwrap_err();
        assert_eq!(err, "Cookie must be in `name=value` form");
    }

    #[test]
    fn test_missing_name() {
        let err = parse_cookie("=value").unwrap_err();
        assert_eq!(err, "Cookie name may not be empty (`=value` is invalid)");
    }

    #[test]
    fn test_disallow_semicolon_in_name() {
        let err = parse_cookie("ke;y=value").unwrap_err();
        assert_eq!(
            err,
            "Cookie name may not contain semicolons or control characters"
        );
    }

    #[test]
    fn test_disallow_semicolon_in_value() {
        let err = parse_cookie("key=val;ue").unwrap_err();
        assert_eq!(
            err,
            "Cookie value may not contain semicolons or control characters"
        );
    }

    #[test]
    fn test_disallow_control_chars() {
        let err = parse_cookie("key=val\r\nue").unwrap_err();
        assert_eq!(
            err,
            "Cookie value may not contain semicolons or control characters"
        );
    }
}
