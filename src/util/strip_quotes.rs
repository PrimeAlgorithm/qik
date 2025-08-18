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
