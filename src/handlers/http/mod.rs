//! Handles HTTP related subcommands execution.

pub mod execute;

use crate::{
    commands::http::{CommonHttpArgs, HttpCommands, PayloadArgs},
    handlers::http::execute::{RequestInformation, execute},
    models::http::Transaction,
};
use reqwest::Method;

/// Execute a parsed [`HttpCommands`] using the given HTTP client
pub async fn execute_http_command(command: &HttpCommands) -> Result<Transaction, anyhow::Error> {
    let result = match command {
        HttpCommands::Get { common } => execute_helper(Method::GET, common, &None).await,
        HttpCommands::Post { common, body } => execute_helper(Method::POST, common, body).await,
        HttpCommands::Put { common, body } => execute_helper(Method::PUT, common, body).await,
        HttpCommands::Delete { common, body } => execute_helper(Method::DELETE, common, body).await,
        HttpCommands::Patch { common, body } => execute_helper(Method::PATCH, common, body).await,
        HttpCommands::Head { common } => execute_helper(Method::HEAD, common, &None).await,
        HttpCommands::Options { common, body } => {
            execute_helper(Method::OPTIONS, common, body).await
        }
    }?;

    Ok(result)
}

// Helper to construct a `RequestInformation` and forward it to `execute`.
async fn execute_helper(
    req_method: Method,
    common: &CommonHttpArgs,
    body: &Option<PayloadArgs>,
) -> Result<Transaction, anyhow::Error> {
    let transaction = execute(RequestInformation {
        method: req_method,
        common: common,
        body: body,
    })
    .await?;

    Ok(transaction)
}
