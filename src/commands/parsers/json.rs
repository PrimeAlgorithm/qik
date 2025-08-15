pub fn parse_json(json: &str) -> Result<String, String> {
    serde_json::from_str::<serde_json::Value>(json)
        .map(|_| json.to_string())
        .map_err(|e| format!("Invalid JSON: {e}"))
}
