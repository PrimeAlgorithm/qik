use reqwest::header::{AUTHORIZATION, COOKIE, HeaderName, PROXY_AUTHORIZATION, SET_COOKIE};

/// Determines if a header is sensitive.
pub fn is_sensitive(name: &HeaderName) -> bool {
    name == AUTHORIZATION
        || name == PROXY_AUTHORIZATION
        || name.as_str().eq_ignore_ascii_case("cookie")
        || name == COOKIE
        || name == SET_COOKIE
}
