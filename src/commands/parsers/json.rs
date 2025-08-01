use serde;

pub fn parse_json(json: &str) -> Result<String, String> {
    let _: serde::de::IgnoredAny = serde_json::from_str(&json).unwrap_or_else(|e| {
        eprintln!("JSON isn't properly formatted\nError: {}", e);
        ::std::process::exit(1);
    });

    Ok(json.to_string())
}
