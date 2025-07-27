use crate::{commands::http::HttpCommands, handlers::http::execute_http_command};
use clap::{Parser, Subcommand};
use reqwest::Client;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Http {
        #[command(subcommand)]
        http_command: HttpCommands,
    },
}

pub async fn execute_cmd(cli: &Cli, http_client: &Client) {
    println!("Executing top level command.");

    match &cli.commands {
        Commands::Http { http_command } => execute_http_command(http_command, http_client).await,
    }
}
