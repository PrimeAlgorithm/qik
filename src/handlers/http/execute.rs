use crate::{
    commands::http::{CommonHttpArgs, PayloadArgs},
    models::http::{RequestSpec, ResponseData, Transaction},
};
use bytes::Bytes;
use reqwest::{
    Client, Method, Version,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};

/// User-supplied request details passed down from the CLI layer.
pub struct RequestInformation<'a> {
    pub method: Method,

    pub common: &'a CommonHttpArgs,

    pub body: &'a Option<PayloadArgs>,
}

/// Send the request and return `(RequestSpec, ResponseData)`.
pub async fn execute(
    client: &Client,
    req_info: RequestInformation<'_>,
) -> Result<Transaction, anyhow::Error> {
    let mut request = client.request(req_info.method.clone(), req_info.common.url.clone());
    let http_version = Version::HTTP_11;
    request = request.version(http_version);

    let mut content_type_header_set = false;
    let mut request_headers = HeaderMap::new();

    if let Some(headers) = &req_info.common.headers {
        for (key, value) in headers {
            if key.as_str().eq_ignore_ascii_case("content-type") {
                content_type_header_set = true;
            }

            request_headers.append(key, value.clone());
        }
    }

    let mut request_body: Option<Bytes> = None;

    // Setup the payload and create content-type if possible.
    if let Some(payload) = req_info.body {
        let mut content_type_hint: Option<&str> = None;

        if let Some(raw) = &payload.raw {
            let body_bytes = Bytes::from(raw.clone());
            request_body = Some(body_bytes.clone());
            request = request.body(body_bytes);
        } else if let Some(json) = &payload.json {
            let body_bytes = Bytes::from(json.clone());
            request_body = Some(body_bytes.clone());
            content_type_hint = Some("application/json");
            request = request.body(body_bytes);
        }

        // TODO: Uncomment once XML parser is setup.
        // else if let Some(xml) = &payload.xml {
        //     let body_bytes = Bytes::from(xml.clone());
        //     request_body = Some(body_bytes.clone());
        //     content_type_hint = Some("application/xml");
        //     request = request.body(body_bytes);
        // }

        if !content_type_header_set {
            if let Some(content_type) = content_type_hint {
                request_headers.insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
            }
        }
    }

    request = request.headers(request_headers.clone());
    let result = request.send().await?;

    Ok((
        RequestSpec {
            method: req_info.method,
            version: http_version,
            url: req_info.common.url.clone(),
            headers: request_headers,
            body: request_body,
        },
        ResponseData {
            status: result.status(),
            version: result.version(),
            headers: result.headers().clone(),
            body: result.bytes().await?,
        },
    ))
}
