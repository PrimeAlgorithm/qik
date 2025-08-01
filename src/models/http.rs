use bytes::Bytes;
use reqwest::{Method, StatusCode, Version, header::HeaderMap};
use url::Url;

#[derive(Debug)]
pub struct RequestSpec {
    pub method: Method,
    pub version: Version,
    pub url: Url,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}

#[derive(Debug)]
pub struct ResponseData {
    pub status: StatusCode,
    pub version: Version,
    pub headers: HeaderMap,
    pub body: Bytes,
}

pub type Transaction = (RequestSpec, ResponseData);
