/// Parses a byte count with an optional binary unit (B, KiB, MiB, GiB).
pub fn parse_byte_size(input: &str) -> Result<usize, String> {
    let input = input.trim();
    let split_at = input
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(input.len());
    let (number, unit) = input.split_at(split_at);

    if number.is_empty() {
        return Err("Size must start with a non-negative integer".to_owned());
    }

    let value = number
        .parse::<usize>()
        .map_err(|_| "Size is too large for this platform".to_owned())?;
    let multiplier = match unit.trim().to_ascii_lowercase().as_str() {
        "" | "b" => 1,
        "kib" | "kb" => 1024,
        "mib" | "mb" => 1024 * 1024,
        "gib" | "gb" => 1024 * 1024 * 1024,
        _ => return Err("Supported size units are B, KiB, MiB, and GiB".to_owned()),
    };

    value
        .checked_mul(multiplier)
        .ok_or_else(|| "Size is too large for this platform".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_sizes() {
        assert_eq!(parse_byte_size("512").unwrap(), 512);
        assert_eq!(parse_byte_size("10MiB").unwrap(), 10 * 1024 * 1024);
        assert_eq!(parse_byte_size("2 gb").unwrap(), 2 * 1024 * 1024 * 1024);
    }

    #[test]
    fn rejects_invalid_or_overflowing_sizes() {
        assert!(parse_byte_size("large").is_err());
        assert!(parse_byte_size("1TB").is_err());
        assert!(parse_byte_size("999999999999999999999999GiB").is_err());
    }
}
