mod cli;
mod commands;
mod handlers;
mod models;
mod output;

use std::io::stdout;

use crate::{cli::execute_cmd, output::printer::Printer};
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let mut printer = Printer::new(stdout());

    if let Err(e) = execute_cmd(&cli, &client, &mut printer).await {
        eprintln!("error: {e:?}");
        std::process::exit(1);
    }
}
