use crate::handlers::http::requests::request_info::RequestInformation;
use reqwest::Certificate;
use std::{fs, io::Read, path::PathBuf};

/// Loads custom CA certs.
pub fn setup_ca_certs(req_info: &RequestInformation) -> Result<Vec<Certificate>, anyhow::Error> {
    let mut certs: Vec<Certificate> = Vec::new();
    if let Some(ca_paths) = &req_info.common.cacert {
        for path in ca_paths.iter() {
            certs.extend(load_ca_certificates(path)?);
        }
    }
    Ok(certs)
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
