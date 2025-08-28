//! Builds and executes requests (with data from [`RequestInformation`])
//!
//! This module turns parsed CLI arguments into a [`reqwest`] request,
//! applies headers, authentication, parameters, and payloads, and
//! returns both the request and response as structured data.

use crate::{
    cli,
    commands::http::{CommonHttpArgs, PayloadArgs},
    models::http::{RequestSpec, ResponseData, Transaction},
};
use base64::prelude::*;
use bytes::Bytes;
use reqwest::{
    Certificate, Client, Identity, Method, Proxy, Version,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
    multipart::{Form, Part},
    redirect::Policy,
};
use std::{collections::HashMap, fs, io::Read, path::PathBuf};

/// User-supplied request details passed down from the CLI layer.
pub struct RequestInformation<'a> {
    pub method: Method,

    pub common: &'a CommonHttpArgs,

    pub body: &'a Option<PayloadArgs>,
}

/// Send the request and return `(RequestSpec, ResponseData)`.
pub async fn execute(req_info: RequestInformation<'_>) -> Result<Transaction, anyhow::Error> {
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

    let mut certs: Vec<Certificate> = Vec::new();
    if let Some(ca_paths) = &req_info.common.cacert {
        for path in ca_paths.iter() {
            certs.extend(load_ca_certificates(path)?);
        }
    }

    let client_identity = load_client_cert(
        &req_info.common.identity_pem,
        &req_info.common.cert,
        &req_info.common.key,
        &req_info.common.p12,
        &req_info.common.p12_pass,
    );

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

    if let Some(time) = req_info.common.timeout {
        request = request.timeout(time);
    }

    if let Some(v) = requested_http_version {
        request = request.version(v);
    }

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

                    let mut part = Part::file(file_path.clone()).await?;
                    if let Some(file_name) = &form_data.file_name {
                        part = part.file_name(file_name.clone()); // this controls the transmitted filename
                    }
                    multipart_form = multipart_form.part(form_data.key.clone(), part);
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

/// Creates a tuple that contains a `Version` (if possible) and a `Client`
/// from a string.
fn build_http_client(
    http_version: &str,
    common: &CommonHttpArgs,
    trusted_certs: Vec<Certificate>,
    client_identity: Option<Identity>,
) -> Result<(Option<reqwest::Version>, Client), anyhow::Error> {
    let (version, mut client_builder) = match http_version {
        "1.0" => (
            Some(Version::HTTP_10),
            reqwest::Client::builder().http1_only(),
        ),
        "1.1" => (
            Some(Version::HTTP_11),
            reqwest::Client::builder().http1_only(),
        ),
        "2" => (
            Some(Version::HTTP_2),
            reqwest::Client::builder().http2_prior_knowledge(),
        ),
        // Let reqwest negotiate automatically
        "auto" => (None, reqwest::Client::builder()),
        _ => (None, reqwest::Client::builder()),
    };

    client_builder = client_builder
        .danger_accept_invalid_certs(common.insecure)
        .danger_accept_invalid_hostnames(common.no_verify_hostname);

    if let Some(redirects) = common.redirects {
        client_builder = client_builder.redirect(Policy::limited(redirects))
    }

    if let Some(proxy) = &common.proxy {
        client_builder = client_builder.proxy(Proxy::all(proxy.as_str())?);
    }

    for cert in trusted_certs.into_iter() {
        client_builder = client_builder.add_root_certificate(cert);
    }

    if let Some(identity) = client_identity {
        client_builder = client_builder.identity(identity);
    }

    return Ok((version, client_builder.build()?));
}

fn load_ca_certificates(path: &PathBuf) -> Result<Vec<Certificate>, anyhow::Error> {
    let mut data = Vec::new();
    fs::File::open(&path)?.read_to_end(&mut data)?;

    let pem_certs = reqwest::Certificate::from_pem_bundle(&data).ok();
    if let Some(certs) = pem_certs {
        return Ok(certs);
    }

    let der_cert = reqwest::Certificate::from_der(&data)?;
    Ok(vec![der_cert])
}

pub fn load_client_cert(
    identity_pem: &Option<PathBuf>,
    cert: &Option<PathBuf>,
    key: &Option<PathBuf>,
    p12: &Option<PathBuf>,
    p12_pass: &Option<String>,
) -> Option<Identity> {
    match (identity_pem, cert, key, p12) {
        (Some(pem), None, None, None) => {
            let bytes = fs::read(pem).ok()?;
            Identity::from_pem(&bytes).ok()
        }

        (None, Some(c), Some(k), None) => {
            let cert_b = fs::read(c).ok()?;
            let key_b = fs::read(k).ok()?;
            Identity::from_pkcs8_pem(&cert_b, &key_b).ok()
        }

        (None, None, None, Some(p)) => {
            let der = fs::read(p).ok()?;
            let pass = p12_pass.as_deref().unwrap_or("");
            Identity::from_pkcs12_der(&der, pass).ok()
        }

        _ => None,
    }
}
