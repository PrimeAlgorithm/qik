//! User-supplied request details passed down from the CLI layer.

use crate::commands::http::{CommonHttpArgs, PayloadArgs};
use reqwest::Method;

/// HTTP request information.
pub struct RequestInformation<'a> {
    pub method: Method,

    pub common: &'a CommonHttpArgs,

    pub body: &'a Option<PayloadArgs>,
}
