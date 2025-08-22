/// Parses a string to ensure it's valid JSON. Returns the given
/// string if it's valid.
///
/// # Errors
/// Returns an error if invalid json is provided.
pub fn parse_json(json: &str) -> Result<String, String> {
    serde_json::from_str::<serde_json::Value>(json)
        .map(|_| json.to_string())
        .map_err(|e| format!("Invalid JSON: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_json() {
        assert_eq!(parse_json("{}").unwrap(), "{}");
    }

    #[test]
    fn test_json() {
        assert_eq!(
            parse_json("{\"key\": \"value\"}").unwrap(),
            "{\"key\": \"value\"}"
        );
    }

    #[test]
    fn test_invalid_json() {
        let err = parse_json("{key: value}").unwrap_err();
        assert!(err.contains("Invalid JSON: "));
    }
}
