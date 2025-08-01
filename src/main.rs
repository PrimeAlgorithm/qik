mod cli;
mod commands;
mod handlers;
mod models;

use crate::cli::execute_cmd;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    execute_cmd(&cli, &client).await;
}
