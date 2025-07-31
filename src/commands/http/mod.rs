use clap::{Args, Subcommand};
use reqwest::header::{HeaderName, HeaderValue};

use crate::commands::parsers::header::parse_header;

#[derive(Args)]
pub struct CommonHttpArgs {
    #[arg(long, value_parser = parse_header)]
    pub headers: Option<Vec<(HeaderName, HeaderValue)>>,
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
    },

    Put {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    Delete {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    Patch {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    Head {
        #[command(flatten)]
        common: CommonHttpArgs,
    },

    Options {
        #[command(flatten)]
        common: CommonHttpArgs,
    },
}
