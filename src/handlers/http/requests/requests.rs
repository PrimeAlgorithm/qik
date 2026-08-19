//! Builds and executes requests (with data from [`RequestInformation`])
//!
//! This module turns parsed CLI arguments into a [`reqwest`] request,
//! applies headers, authentication, parameters, and payloads, and
//! returns both the request and response as structured data.

use crate::{
    error::{ErrorKind, QikError},
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
use bytes::{Bytes, BytesMut};
use reqwest::{
    header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue},
    multipart::Form,
};
use std::io::Write;

const DEFAULT_FORMATTED_RESPONSE_LIMIT: usize = 10 * 1024 * 1024;

/// Send the request and return `(RequestSpec, ResponseData)`.
pub async fn request(
    req_info: RequestInformation<'_>,
    mut response_body_sink: Option<&mut dyn Write>,
) -> Result<Transaction, anyhow::Error> {
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
    let certs = setup_ca_certs(&req_info).map_err(|error| QikError::new(ErrorKind::Tls, error))?;
    let client_identity =
        load_client_cert(&req_info).map_err(|error| QikError::new(ErrorKind::Tls, error))?;

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
    )
    .map_err(|error| QikError::new(ErrorKind::Transport, error))?;

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
    let request_body;
    request_headers = set_cookies(&req_info, request_headers)?;

    match set_payload(req_info.body).await? {
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
        Payload::None => {
            request = request.body(Bytes::from(""));
            request_headers.append(CONTENT_LENGTH, HeaderValue::from_static("0"));
            request_body = Payload::None;
        }
    };

    let auth_info = load_auth(&req_info)?;
    if let Some((header_name, header_value)) = auth_info {
        request_headers.append(header_name, header_value);
    }

    mark_sensitive_headers(
        &mut request_headers,
        req_info.common.redact_header.as_deref(),
    );

    request = request.headers(request_headers.clone());
    let mut result = request.send().await.map_err(|error| {
        QikError::from_reqwest(
            error,
            &format!("failed to send {} request to {url}", req_info.method),
        )
    })?;
    let negotiated = result.version();
    let status = result.status();
    let mut response_headers = result.headers().clone();
    mark_sensitive_headers(
        &mut response_headers,
        req_info.common.redact_header.as_deref(),
    );

    let streaming = response_body_sink.is_some();
    let max_size = req_info.common.max_response_size.unwrap_or(if streaming {
        0
    } else {
        DEFAULT_FORMATTED_RESPONSE_LIMIT
    });
    if max_size != 0
        && result
            .content_length()
            .is_some_and(|content_length| content_length > max_size as u64)
    {
        return Err(response_too_large(max_size).into());
    }

    let mut response_body = BytesMut::new();
    let mut received = 0usize;
    while let Some(chunk) = result
        .chunk()
        .await
        .map_err(|error| QikError::from_reqwest(error, "failed while reading response body"))?
    {
        received = received.saturating_add(chunk.len());
        if max_size != 0 && received > max_size {
            return Err(response_too_large(max_size).into());
        }

        if let Some(sink) = response_body_sink.as_deref_mut() {
            sink.write_all(&chunk)
                .map_err(|error| QikError::new(ErrorKind::Output, error))?;
        } else {
            response_body.extend_from_slice(&chunk);
        }
    }

    if let Some(sink) = response_body_sink.as_deref_mut() {
        sink.flush()
            .map_err(|error| QikError::new(ErrorKind::Output, error))?;
    }

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
            status,
            version: negotiated,
            headers: response_headers,
            body: response_body.freeze(),
        },
    ))
}

fn response_too_large(max_size: usize) -> QikError {
    QikError::new(
        ErrorKind::ResponseTooLarge,
        anyhow::anyhow!(
            "response body exceeds the configured limit of {max_size} bytes (use --max-response-size to change it)"
        ),
    )
}

fn mark_sensitive_headers(headers: &mut HeaderMap, extra_sensitive: Option<&[HeaderName]>) {
    let Some(extra_sensitive) = extra_sensitive else {
        return;
    };

    for (name, value) in headers.iter_mut() {
        if extra_sensitive.iter().any(|sensitive| sensitive == name) {
            value.set_sensitive(true);
        }
    }
}
