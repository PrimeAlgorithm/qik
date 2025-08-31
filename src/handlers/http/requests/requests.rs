//! Builds and executes requests (with data from [`RequestInformation`])
//!
//! This module turns parsed CLI arguments into a [`reqwest`] request,
//! applies headers, authentication, parameters, and payloads, and
//! returns both the request and response as structured data.

use crate::{
    handlers::http::requests::{
        build_client::build_http_client,
        load_auth::load_auth,
        load_ca_certs::setup_ca_certs,
        load_client_cert::load_client_cert,
        request_info::RequestInformation,
        set_cookies::set_cookies,
        set_headers::set_headers,
        set_payload::{BodyInfo, Payload, set_payload},
    },
    models::http::{RequestSpec, ResponseData, Transaction},
};
use reqwest::{
    header::{CONTENT_TYPE, HeaderValue},
    multipart::Form,
};

/// Send the request and return `(RequestSpec, ResponseData)`.
pub async fn request(req_info: RequestInformation<'_>) -> Result<Transaction, anyhow::Error> {
    if req_info.common.insecure || req_info.common.no_verify_hostname {
        use owo_colors::OwoColorize;
        eprintln!(
            "{}",
            "WARNING: Certificate verification is DISABLED or PARTIALLY DISABLED.\n\
         The server's identity is NOT verified.\n\
         Use only on trusted networks. "
                .yellow()
                .bold()
        );
    }

    let mut url = req_info.common.url.clone();
    let certs = setup_ca_certs(&req_info)?;
    let client_identity = load_client_cert(&req_info);

    // If the user has added params, create a new URL with it.
    if let Some(params) = &req_info.common.param {
        url.query_pairs_mut().extend_pairs(params.iter());
    }

    // Build the client.
    let (requested_http_version, client) = build_http_client(
        req_info.common.http_version.as_ref(),
        req_info.common,
        certs,
        client_identity,
    )?;

    // Setup the request.
    let mut request = client.request(req_info.method.clone(), url.clone());

    // Add timeout to request.
    if let Some(time) = req_info.common.timeout {
        request = request.timeout(time);
    }

    // Add http version to the request.
    if let Some(v) = requested_http_version {
        request = request.version(v);
    }

    let mut request_headers = set_headers(&req_info)?;
    let mut request_body = Payload::None;
    request_headers = set_cookies(&req_info, request_headers)?;

    if let Some(payload_data) = req_info.body {
        match set_payload(&payload_data).await? {
            Payload::Body(data) => {
                request = request.body(data.content.clone());

                if !request_headers.contains_key(CONTENT_TYPE) {
                    if let Some(content_type) = data.content_type.clone() {
                        request_headers.append(CONTENT_TYPE, HeaderValue::from_str(&content_type)?);
                    }
                }

                request_body = Payload::Body(BodyInfo {
                    content: data.content.clone(),
                    content_type: data.content_type.clone(),
                });
            }
            Payload::Form(params) => {
                request = request.form(&params);
                request_body = Payload::Form(params.clone());
            }
            Payload::Multipart(form) => {
                request = request.multipart(form);
                request_body = Payload::Multipart(Form::new());
            }
            Payload::None => request_body = Payload::None,
        };
    }

    let auth_info = load_auth(&req_info)?;
    if let Some((header_name, header_value)) = auth_info {
        request_headers.append(header_name, header_value);
    }

    request = request.headers(request_headers.clone());
    let result = request.send().await?;
    let negotiated = result.version();

    Ok((
        RequestSpec {
            method: req_info.method,
            version: requested_http_version,
            negotiated: negotiated,
            url: url.clone(),
            headers: request_headers,
            body: request_body,
        },
        ResponseData {
            status: result.status(),
            version: negotiated,
            headers: result.headers().clone(),
            body: result.bytes().await?,
        },
    ))
}
