//! CLI parsing and command dispatch.

use crate::{
    commands::http::HttpCommands,
    error::{ErrorKind, QikError},
    handlers::http::requests::execute::execute_http_command,
    output::{
        format::formatter::{format_request, format_response, format_transaction},
        printer::Printer,
    },
};
use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write;

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum OutputMode {
    /// Print both the request and response.
    #[default]
    All,
    /// Print only the request.
    Request,
    /// Print only the response status, headers, and body.
    Response,
    /// Print only the response body (useful in scripts).
    Body,
}

/// This struct is the root of the CLI tree, which is used
/// by [`clap`] for processing commands. It contains all the
/// available subcommands via `commands`.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Select which part of the HTTP transaction to print.
    #[arg(long, global = true, value_enum, default_value_t)]
    pub output: OutputMode,

    /// Disable ANSI colors (useful when redirecting output).
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Exit unsuccessfully when the response status is 4xx or 5xx.
    #[arg(long, global = true)]
    pub check_status: bool,

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
pub async fn execute_cmd<W: Write>(cli: &Cli, printer: &mut Printer<W>) -> anyhow::Result<()> {
    match &cli.commands {
        Commands::Http { http_command } => {
            let response_body_sink = if matches!(cli.output, OutputMode::Body) {
                Some(printer as &mut dyn Write)
            } else {
                None
            };
            let transaction = execute_http_command(http_command, response_body_sink).await?;
            let status = transaction.1.status;

            match cli.output {
                OutputMode::All => {
                    let (req, res) = format_transaction(transaction)?;
                    printer.println(&req).map_err(output_error)?;
                    printer.print("\n").map_err(output_error)?;
                    printer.println(&res).map_err(output_error)?;
                }
                OutputMode::Request => printer
                    .println(&format_request(&transaction.0)?)
                    .map_err(output_error)?,
                OutputMode::Response => printer
                    .println(&format_response(&transaction.1)?)
                    .map_err(output_error)?,
                OutputMode::Body => {}
            }

            if cli.check_status && (status.is_client_error() || status.is_server_error()) {
                printer.flush().map_err(output_error)?;
                return Err(QikError::new(
                    ErrorKind::HttpStatus,
                    anyhow::anyhow!("server returned HTTP {status}"),
                )
                .into());
            }

            Ok(())
        }
    }
}

fn output_error(error: std::io::Error) -> QikError {
    QikError::new(ErrorKind::Output, error)
}
