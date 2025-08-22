/// Removes matching single or double quotes around a string slice.
/// If there are no matching quotes found, the string is returned
/// as is.
pub fn strip_matching_quotes(s: &str) -> &str {
    if s.len() >= 2 {
        let bytes = s.as_bytes();
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        let matching_double = first == b'"' && last == b'"';
        let matching_single = first == b'\'' && last == b'\'';
        if matching_double || matching_single {
            return &s[1..s.len() - 1];
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_double_quotes() {
        let quotes_stripped = strip_matching_quotes("\"test\"");
        assert_eq!(quotes_stripped, "test");
    }

    #[test]
    fn test_strip_single_quotes() {
        let quotes_stripped = strip_matching_quotes("'test'");
        assert_eq!(quotes_stripped, "test");
    }

    #[test]
    fn test_non_matching_double_quotes() {
        let s = strip_matching_quotes("\"test");
        assert_eq!(s, "\"test");
    }

    #[test]
    fn test_non_matching_single_quotes() {
        let s = strip_matching_quotes("'test");
        assert_eq!(s, "'test");
    }
}
