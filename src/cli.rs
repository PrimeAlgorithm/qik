use std::io::Write;

use crate::{
    commands::http::HttpCommands,
    handlers::http::execute_http_command,
    output::{formatter::format_transaction, printer::Printer},
};
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

pub async fn execute_cmd<W: Write>(
    cli: &Cli,
    http_client: &Client,
    printer: &mut Printer<W>,
) -> anyhow::Result<()> {
    match &cli.commands {
        Commands::Http { http_command } => {
            let transaction = execute_http_command(http_command, http_client).await?;
            let (req, res) = format_transaction(transaction)?;

            printer.println(&req)?;
            printer.print("\n")?;
            printer.println(&res)?;

            Ok(())
        }
    }
}
