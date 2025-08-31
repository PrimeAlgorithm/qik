use owo_colors::OwoColorize;
use reqwest::StatusCode;

/// Returns colored string representation of a status code.
pub fn status_code_color(code: StatusCode) -> String {
    let styled = match code.as_u16() {
        100..=199 => format!("{}", code.as_str().cyan().bold()),
        200..=299 => format!("{}", code.as_str().green().bold()),
        300..=399 => format!("{}", code.as_str().yellow().bold()),
        400..=499 => format!("{}", code.as_str().red().bold()),
        _ => format!("{}", code.as_str().bright_red().bold()),
    };

    styled
}
