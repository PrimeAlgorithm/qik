use clap::{ArgGroup, Args, Subcommand};
use reqwest::header::{HeaderName, HeaderValue};
use url::Url;

use crate::commands::parsers::{header::parse_header, json::parse_json};

#[derive(Args)]
pub struct CommonHttpArgs {
    pub url: Url,

    #[arg(long, value_parser = parse_header)]
    pub headers: Option<Vec<(HeaderName, HeaderValue)>>,
}

#[derive(Args)]
#[command(
    group(
        ArgGroup::new("payload")
            .args(["raw", "json", "xml"])
            .multiple(false)
    )
)]
pub struct PayloadArgs {
    #[arg(long, short)]
    pub raw: Option<String>,

    #[arg(long, short, value_parser = parse_json)]
    pub json: Option<String>,
    // Uncomment once XML parser is setup.
    // #[arg(long, short)]
    // pub xml: Option<String>,
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
