use crate::output::format::is_sensitive::is_sensitive;
use owo_colors::OwoColorize;
use reqwest::header::{AUTHORIZATION, HeaderMap, PROXY_AUTHORIZATION};

/// Return header block (`\nKey: Value…`) or empty string.
pub fn format_headers(headers: &HeaderMap) -> Result<String, anyhow::Error> {
    let mut out = String::new();

    for (name, value) in headers {
        let key = name.bright_black();

        if value.is_sensitive() {
            out.push_str(&format!("\n{key}: <redacted>"));
            continue;
        }

        if name == AUTHORIZATION || name == PROXY_AUTHORIZATION {
            let s = value.to_str().unwrap_or_default();
            let scheme = s.splitn(2, char::is_whitespace).next().unwrap_or("");
            if scheme.is_empty() {
                out.push_str(&format!("\n{key}: <redacted>"));
            } else {
                out.push_str(&format!("\n{key}: {scheme} <redacted>"));
            }
            continue;
        }

        if is_sensitive(name) {
            out.push_str(&format!("\n{key}: <redacted>"));
            continue;
        }

        match value.to_str() {
            Ok(v) => out.push_str(&format!("\n{key}: {v}")),
            Err(_) => out.push_str(&format!(
                "\n{key}: <non-UTF8 value ({} bytes)>",
                value.as_bytes().len()
            )),
        }
    }

    Ok(out)
}
