//! HTTP subcommands and their argument structures.

use std::path::PathBuf;

use crate::commands::parsers::{
    auth::parse_auth,
    bearer::parse_bearer,
    form::{FormData, parse_form},
    header::parse_header,
    json::parse_json,
    param::parse_param,
    xml::parse_xml,
};
use clap::{ArgGroup, Args, Subcommand, command};
use humantime::parse_duration;
use reqwest::header::{HeaderName, HeaderValue};
use url::Url;

#[derive(Args)]
#[command(
    group(
        ArgGroup::new("authorization")
            .args(["auth", "bearer"])
            .multiple(false)
    )
)]
pub struct CommonHttpArgs {
    /// Target URL for the request.
    pub url: Url,

    /// Repeatable header in `Key: Value` form. Can be provided multiple times.
    #[arg(long, value_parser = parse_header)]
    pub header: Option<Vec<(HeaderName, HeaderValue)>>,

    /// Repeatable parameters in `key=value` form. Can be provided multiple times.
    #[arg(long, short, value_parser = parse_param)]
    pub param: Option<Vec<(String, String)>>,

    /// Basic auth credentials in `user:pass` form.
    #[arg(long, short, value_parser = parse_auth)]
    pub auth: Option<String>,

    /// Bearer token (matching quotes are removed from bearer value).
    #[arg(long, short, value_parser = parse_bearer)]
    pub bearer: Option<String>,

    #[
        arg(long = "http-version", short = 'H', 
        value_parser = ["auto", "1.0", "1.1", "2"], 
        default_value = "auto")
    ]
    pub http_version: String,

    /// DANGEROUS:
    /// Will allow connections to proceed even if the server’s certificate is invalid.
    /// Using this may open you to vulnerabilities and attacks.
    #[arg(long)]
    pub insecure: bool,

    /// DANGEROUS:
    /// Will allow connections to proceed even if the server’s hostname is invalid.
    /// Using this may open you to vulnerabilities and attacks.
    #[arg(long = "no-verify-hostname")]
    pub no_verify_hostname: bool,

    /// Adds trusted certificate authority certificates.
    /// Repeatable: pass --cacert multiple times or use a PEM bundle.
    #[arg(long)]
    pub cacert: Option<Vec<PathBuf>>,

    #[arg(long = "identity-pem", conflicts_with_all = ["cert", "key"])]
    pub identity_pem: Option<PathBuf>,

    /// Client certificate chain in PEM (leaf first, then intermediates).
    /// Use with `--key`.
    #[arg(long, requires = "key")]
    pub cert: Option<PathBuf>,

    /// Matching client private key in PKCS#8 PEM.
    /// Use with `--cert`.
    #[arg(long, requires = "cert")]
    pub key: Option<PathBuf>,

    /// PKCS#12/.pfx containing client key and cert chain.
    /// Use with `--p12-pass`.
    #[arg(long, requires = "p12_pass")]
    pub p12: Option<PathBuf>,

    /// Password for the `--p12` PKCS#12 archive (use empty string if none).
    #[arg(long = "p12-pass", requires = "p12")]
    pub p12_pass: Option<String>,

    /// If the request takes longer than the specified timeout
    /// the request will fail with a timeout error.
    #[arg(long, value_parser = parse_duration)]
    pub timeout: Option<std::time::Duration>,
}

/// Optional request body for methods that support one.
///
/// Exactly one of `--raw`, `--json`, `--xml`, or `--form` may be provided.
#[derive(Args)]
#[command(
    group(
        ArgGroup::new("payload")
            .args(["raw", "json", "xml", "form"])
            .multiple(false)
    )
)]
pub struct PayloadArgs {
    /// Send the literal string as the request body (no content-type set).
    #[arg(long, short)]
    pub raw: Option<String>,

    /// Valdiates and sends JSON provided as is. Implies `Content-Type: application/json`
    /// unless content-type is provided via `--header`.
    #[arg(long, short, value_parser = parse_json)]
    pub json: Option<String>,

    /// Validates and sends XML as is. Implies `Content-Type: application/xml` unless
    /// content-type header is provided via `--header`.
    #[arg(long, short, value_parser = parse_xml)]
    pub xml: Option<String>,

    /// Repeatable form fields to send with the request. `key=value` (text) or
    /// `key=@path[;filename=name]` (file) is required. If a file is present, multipart
    /// is used, otherwise the application defaults to `application/x-www-form-urlencoded`
    #[arg(long, short, value_parser = parse_form)]
    pub form: Option<Vec<FormData>>,
}

/// Top level HTTP verb definitions.
#[derive(Subcommand)]
pub enum HttpCommands {
    /// Issue a GET request.
    Get {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    /// Issue a POST request.
    Post {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    /// Issue a PUT request.
    Put {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    /// Issue a DELETE request.
    Delete {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    /// Issue a PATCH request.
    Patch {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    /// Issue a HEAD request.
    Head {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    /// Issue an options request.
    Options {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },
}
