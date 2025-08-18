//! CLI parsing and command dispatch.

use crate::{
    commands::http::HttpCommands,
    handlers::http::execute_http_command,
    output::{formatter::format_transaction, printer::Printer},
};
use clap::{Parser, Subcommand};
use reqwest::Client;
use std::io::Write;

/// This struct is the root of the CLI tree, which is used
/// by [`clap`] for processing commands. It contains all the
/// available subcommands via `commands`.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub commands: Commands,
}

/// All supported top level commands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
    Http {
        #[command(subcommand)]
        http_command: HttpCommands,
    },
}

/// Matches a processed command (via [`clap`]) to appropriate handler
/// and outputs results to writer.
///
/// # Errors
/// Returns an error if the command execution or formatting fails.
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
