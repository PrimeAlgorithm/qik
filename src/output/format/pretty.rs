/// Returns pretty-print JSON bodies.
pub fn pretty_json(json_str: &str) -> Result<String, anyhow::Error> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;
    let formatted_json = serde_json::to_string_pretty(&json_value)?;

    Ok(formatted_json)
}
