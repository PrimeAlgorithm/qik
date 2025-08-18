use clap::{ArgGroup, Args, Subcommand};
use reqwest::header::{HeaderName, HeaderValue};
use url::Url;

use crate::commands::parsers::{
    auth::parse_auth,
    bearer::parse_bearer,
    form::{FormData, parse_form},
    header::parse_header,
    json::parse_json,
    param::parse_param,
};

#[derive(Args)]
#[command(
    group(
        ArgGroup::new("payload")
            .args(["auth", "bearer"])
            .multiple(false)
    )
)]
pub struct CommonHttpArgs {
    pub url: Url,

    #[arg(long, value_parser = parse_header)]
    pub header: Option<Vec<(HeaderName, HeaderValue)>>,

    #[arg(long, short, value_parser = parse_param)]
    pub param: Option<Vec<(String, String)>>,

    #[arg(long, short, value_parser = parse_auth)]
    pub auth: Option<String>,

    #[arg(long, short, value_parser = parse_bearer)]
    pub bearer: Option<String>,
}

#[derive(Args)]
#[command(
    group(
        ArgGroup::new("payload")
            .args(["raw", "json", "xml", "form"])
            .multiple(false)
    )
)]
pub struct PayloadArgs {
    #[arg(long, short)]
    pub raw: Option<String>,

    #[arg(long, short, value_parser = parse_json)]
    pub json: Option<String>,

    #[arg(long, short)]
    pub xml: Option<String>,

    #[arg(long, short, value_parser = parse_form)]
    pub form: Option<Vec<FormData>>,
}

#[derive(Subcommand)]
pub enum HttpCommands {
    Get {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    Post {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    Put {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    Delete {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    Patch {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },

    Head {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    Options {
        #[command(flatten)]
        common: CommonHttpArgs,

        #[command(flatten)]
        body: Option<PayloadArgs>,
    },
}
