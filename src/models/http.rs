//! Contains data models used to represent HTTP requests and responses.

use crate::handlers::http::requests::set_payload::Payload;
use bytes::Bytes;
use reqwest::{Method, StatusCode, Version, header::HeaderMap};
use url::Url;

/// A structured representation of an HTTP request.
pub struct RequestSpec {
    /// HTTP verb.
    pub method: Method,

    /// What the user requested. `None` means "auto" (let the client negotiate).
    pub version: Option<Version>,

    /// What actually happened on the wire (same as ResponseData.version).
    pub negotiated: Version,

    /// Target url.
    pub url: Url,

    /// Request headers.
    pub headers: HeaderMap,

    /// Optional request body.
    pub body: Payload,
}

/// A structured representation of an HTTP response.
#[derive(Debug)]
pub struct ResponseData {
    /// HTTP status code (e.g. 200 OK).
    pub status: StatusCode,

    /// HTTP protocol version.
    pub version: Version,

    /// Response headers.
    pub headers: HeaderMap,

    /// Response body.
    pub body: Bytes,
}

/// A request/response pair: (`RequestSpec`, `ResponseData`).
pub type Transaction = (RequestSpec, ResponseData);
