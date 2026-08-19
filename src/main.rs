//! Entry point for the CLI application.
//! Parses arguments, builds an HTTP client, and delegates to the command layer.

mod cli;
mod commands;
mod error;
mod handlers;
mod models;
mod output;
mod util;

use crate::{cli::execute_cmd, error::QikError, output::printer::Printer};
use clap::Parser;
use cli::Cli;
use std::{
    env,
    io::{IsTerminal, stdout},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let no_color_environment = env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty());
    if cli.no_color || no_color_environment || !stdout().is_terminal() {
        owo_colors::set_override(false);
    }

    let mut printer = Printer::new(stdout());

    if let Err(e) = execute_cmd(&cli, &mut printer).await {
        eprintln!("error: {e:#}");
        let exit_code = e
            .chain()
            .find_map(|cause| cause.downcast_ref::<QikError>())
            .map_or(1, QikError::exit_code);
        std::process::exit(exit_code);
    }
}
