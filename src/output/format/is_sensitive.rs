use reqwest::header::{AUTHORIZATION, COOKIE, HeaderName, PROXY_AUTHORIZATION, SET_COOKIE};

/// Determines if a header is sensitive.
pub fn is_sensitive(name: &HeaderName) -> bool {
    name == AUTHORIZATION || name == PROXY_AUTHORIZATION || name == COOKIE || name == SET_COOKIE
}
