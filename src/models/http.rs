//! Contains data models used to represent HTTP requests and responses.

use bytes::Bytes;
use reqwest::{Method, StatusCode, Version, header::HeaderMap};
use url::Url;

/// A structured representation of an HTTP request.
#[derive(Debug)]
pub struct RequestSpec {
    /// HTTP verb.
    pub method: Method,

    /// Version of HTTP protocol being used.
    pub version: Version,

    /// Target url.
    pub url: Url,

    /// Request headers..
    pub headers: HeaderMap,

    /// Optional request body.
    pub body: Option<Bytes>,
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
