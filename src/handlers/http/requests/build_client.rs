use crate::commands::http::CommonHttpArgs;
use reqwest::{Certificate, Client, Identity, Proxy, Version, redirect::Policy};

/// Creates a tuple that contains a [`Version`] (if possible) and a [`Client`]
/// from a string.
pub fn build_http_client(
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
        "2" => (Some(Version::HTTP_2), reqwest::Client::builder()),
        // Let reqwest negotiate automatically
        "auto" => (None, reqwest::Client::builder()),
        _ => (None, reqwest::Client::builder()),
    };

    client_builder = client_builder
        .danger_accept_invalid_certs(common.insecure)
        .danger_accept_invalid_hostnames(common.no_verify_hostname);

    if let Some(redirects) = common.redirects {
        client_builder = client_builder.redirect(if redirects == 0 {
            Policy::none()
        } else {
            Policy::limited(redirects)
        });
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
