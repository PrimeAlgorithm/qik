//! Builds and executes requests (with data from [`RequestInformation`])
//!
//! This module turns parsed CLI arguments into a [`reqwest`] request,
//! applies headers, authentication, parameters, and payloads, and
//! returns both the request and response as structured data.

use crate::{
    commands::http::{CommonHttpArgs, PayloadArgs},
    models::http::{RequestSpec, ResponseData, Transaction},
};
use anyhow::anyhow;
use base64::prelude::*;
use bytes::Bytes;
use reqwest::{
    Client, Method, Version,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
    multipart::Form,
};
use std::collections::HashMap;

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
    let mut url = req_info.common.url.clone();

    // If the user has added params, create a new URL with it.
    if let Some(params) = &req_info.common.param {
        url.query_pairs_mut().extend_pairs(params.iter());
    }

    // Setup the request.
    let mut request = client.request(req_info.method.clone(), url.clone());
    let http_version = Version::HTTP_11;
    request = request.version(http_version);

    let mut content_type_header_set = false;
    let mut request_headers = HeaderMap::new();

    if let Some(headers) = &req_info.common.header {
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
        } else if let Some(form_data_list) = &payload.form {
            let mut url_params = HashMap::new();
            let mut multipart_form = Form::new();
            let mut use_multipart = false;

            for form_data in form_data_list.iter() {
                if let Some(file_path) = &form_data.file_path {
                    use_multipart = true;

                    if let Some(file_name) = &form_data.file_name {
                        multipart_form = multipart_form
                            .file(file_name.clone(), file_path.clone())
                            .await?;
                    } else {
                        let name = file_path
                            .file_name()
                            .ok_or_else(|| anyhow!("Failed to get file name from file path"))?
                            .to_str()
                            .ok_or_else(|| anyhow!("Failed to convert file name to string"))?
                            .to_owned();
                        multipart_form = multipart_form.file(name, file_path.clone()).await?;
                    }
                } else if let Some(str) = &form_data.str_value {
                    url_params.insert(form_data.key.clone(), str.clone());
                }
            }

            if use_multipart {
                for (key, value) in url_params.into_iter() {
                    multipart_form = multipart_form.text(key, value);
                }
                request = request.multipart(multipart_form);
            } else {
                request = request.form(&url_params);
            }
        } else if let Some(xml) = &payload.xml {
            let body_bytes = Bytes::from(xml.clone());
            request_body = Some(body_bytes.clone());
            content_type_hint = Some("application/xml");
            request = request.body(body_bytes);
        }

        if !content_type_header_set {
            if let Some(content_type) = content_type_hint {
                request_headers.insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
            }
        }
    }

    if let Some(user_credentials) = &req_info.common.auth {
        let credentials_encoded = BASE64_STANDARD.encode(user_credentials);
        let formatted_auth_header = format!("Basic {}", credentials_encoded);

        request_headers.append(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&formatted_auth_header)?,
        );
    } else if let Some(bearer_token) = &req_info.common.bearer {
        let formatted_bearer_token = format!("Bearer {}", bearer_token);

        request_headers.append(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&formatted_bearer_token)?,
        );
    }

    request = request.headers(request_headers.clone());
    let result = request.send().await?;

    Ok((
        RequestSpec {
            method: req_info.method,
            version: http_version,
            url: url.clone(),
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
