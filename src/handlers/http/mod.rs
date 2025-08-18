//! Handles HTTP related subcommands execution.

pub mod execute;

use crate::{
    commands::http::{CommonHttpArgs, HttpCommands, PayloadArgs},
    handlers::http::execute::{RequestInformation, execute},
    models::http::Transaction,
};
use reqwest::{Client, Method};

/// Execute a parsed [`HttpCommands`] using the given HTTP client
pub async fn execute_http_command(
    command: &HttpCommands,
    client: &Client,
) -> Result<Transaction, anyhow::Error> {
    let result = match command {
        HttpCommands::Get { common } => execute_helper(client, Method::GET, common, &None).await,
        HttpCommands::Post { common, body } => {
            execute_helper(client, Method::POST, common, body).await
        }
        HttpCommands::Put { common, body } => {
            execute_helper(client, Method::PUT, common, body).await
        }
        HttpCommands::Delete { common, body } => {
            execute_helper(client, Method::DELETE, common, body).await
        }
        HttpCommands::Patch { common, body } => {
            execute_helper(client, Method::PATCH, common, body).await
        }
        HttpCommands::Head { common } => execute_helper(client, Method::HEAD, common, &None).await,
        HttpCommands::Options { common, body } => {
            execute_helper(client, Method::OPTIONS, common, body).await
        }
    }?;

    Ok(result)
}

// Helper to construct a `RequestInformation` and forward it to `execute`.
async fn execute_helper(
    client: &Client,
    req_method: Method,
    common: &CommonHttpArgs,
    body: &Option<PayloadArgs>,
) -> Result<Transaction, anyhow::Error> {
    let transaction = execute(
        client,
        RequestInformation {
            method: req_method,
            common: common,
            body: body,
        },
    )
    .await?;

    Ok(transaction)
}
