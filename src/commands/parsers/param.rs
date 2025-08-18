/// Parses a `key=value` parameter string.
///
/// Splits the input on the first `=` into `(key, value)`.
///
/// # Errors
/// Returns an error if no `=` is present or if the key is empty.
pub fn parse_param(param: &str) -> Result<(String, String), String> {
    let (key, value) = param
        .split_once('=')
        .ok_or_else(|| "Parameter must be in `key=value` form")?;

    if key.is_empty() {
        return Err("parameter key may not be empty (`=value` is invalid)".to_owned());
    }

    Ok((key.to_owned(), value.to_owned()))
}
