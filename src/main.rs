//! Entry point for the CLI application.
//! Parses arguments, builds an HTTP client, and delegates to the command layer.

mod cli;
mod commands;
mod handlers;
mod models;
mod output;
mod util;

use crate::{cli::execute_cmd, output::printer::Printer};
use clap::Parser;
use cli::Cli;
use std::io::stdout;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut printer = Printer::new(stdout());

    if let Err(e) = execute_cmd(&cli, &mut printer).await {
        eprintln!("error: {e:?}");
        std::process::exit(1);
    }
}
