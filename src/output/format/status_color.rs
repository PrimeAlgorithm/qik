use owo_colors::OwoColorize;
use reqwest::StatusCode;

/// Returns colored string representation of a status code.
pub fn status_code_color(code: StatusCode) -> String {
    let status = match code.canonical_reason() {
        Some(reason) => format!("{} {reason}", code.as_str()),
        None => code.as_str().to_owned(),
    };
    let styled = match code.as_u16() {
        100..=199 => format!("{}", status.cyan().bold()),
        200..=299 => format!("{}", status.green().bold()),
        300..=399 => format!("{}", status.yellow().bold()),
        400..=499 => format!("{}", status.red().bold()),
        _ => format!("{}", status.bright_red().bold()),
    };

    styled
}
