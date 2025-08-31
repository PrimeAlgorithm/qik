use crate::handlers::http::requests::request_info::RequestInformation;
use reqwest::Identity;
use std::fs;

/// Loads client certificate for mutual TLS auth.
pub fn load_client_cert(req_info: &RequestInformation) -> Option<Identity> {
    let (identity_pem, cert, key, p12, p12_pass) = (
        &req_info.common.identity_pem,
        &req_info.common.cert,
        &req_info.common.key,
        &req_info.common.p12,
        &req_info.common.p12_pass,
    );

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
