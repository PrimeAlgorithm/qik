use anyhow::{Context, bail};
use crate::handlers::http::requests::request_info::RequestInformation;
use reqwest::Identity;
use std::fs;

/// Loads client certificate for mutual TLS auth.
pub fn load_client_cert(req_info: &RequestInformation) -> anyhow::Result<Option<Identity>> {
    let (identity_pem, cert, key, p12, p12_pass) = (
        &req_info.common.identity_pem,
        &req_info.common.cert,
        &req_info.common.key,
        &req_info.common.p12,
        &req_info.common.p12_pass,
    );

    match (identity_pem, cert, key, p12) {
        (Some(pem), None, None, None) => {
            let bytes = fs::read(pem)
                .with_context(|| format!("failed to read PEM identity from {}", pem.display()))?;
            Ok(Some(
                Identity::from_pem(&bytes).context("invalid PEM identity")?,
            ))
        }

        (None, Some(c), Some(k), None) => {
            let cert_b = fs::read(c).with_context(|| {
                format!("failed to read client certificate from {}", c.display())
            })?;
            let key_b = fs::read(k)
                .with_context(|| format!("failed to read client key from {}", k.display()))?;
            Ok(Some(
                Identity::from_pkcs8_pem(&cert_b, &key_b)
                    .context("invalid client certificate or PKCS#8 key")?,
            ))
        }

        (None, None, None, Some(p)) => {
            let der = fs::read(p)
                .with_context(|| format!("failed to read PKCS#12 identity from {}", p.display()))?;
            let pass = p12_pass.as_deref().unwrap_or("");
            Ok(Some(Identity::from_pkcs12_der(&der, pass).context(
                "invalid PKCS#12 identity or password",
            )?))
        }

        (None, None, None, None) => Ok(None),
        _ => bail!("invalid client identity option combination"),
    }
}
