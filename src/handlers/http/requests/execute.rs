//! Handles HTTP command mapping and execution.

use crate::{
    commands::http::{CommonHttpArgs, HttpCommands, PayloadArgs},
    handlers::http::requests::{request_info::RequestInformation, requests::request},
    models::http::Transaction,
};
use reqwest::Method;
use std::io::Write;

/// Execute a parsed [`HttpCommands`] using the given HTTP client
pub async fn execute_http_command(
    command: &HttpCommands,
    response_body_sink: Option<&mut dyn Write>,
) -> Result<Transaction, anyhow::Error> {
    let result = match command {
        HttpCommands::Get { common } => {
            execute_helper(Method::GET, common, &None, response_body_sink).await
        }
        HttpCommands::Post { common, body } => {
            execute_helper(Method::POST, common, body, response_body_sink).await
        }
        HttpCommands::Put { common, body } => {
            execute_helper(Method::PUT, common, body, response_body_sink).await
        }
        HttpCommands::Delete { common, body } => {
            execute_helper(Method::DELETE, common, body, response_body_sink).await
        }
        HttpCommands::Patch { common, body } => {
            execute_helper(Method::PATCH, common, body, response_body_sink).await
        }
        HttpCommands::Head { common } => {
            execute_helper(Method::HEAD, common, &None, response_body_sink).await
        }
        HttpCommands::Options { common, body } => {
            execute_helper(Method::OPTIONS, common, body, response_body_sink).await
        }
    }?;

    Ok(result)
}

// Helper to construct a `RequestInformation` and forward it to `execute`.
async fn execute_helper(
    req_method: Method,
    common: &CommonHttpArgs,
    body: &Option<PayloadArgs>,
    response_body_sink: Option<&mut dyn Write>,
) -> Result<Transaction, anyhow::Error> {
    let transaction = request(
        RequestInformation {
            method: req_method,
            common,
            body,
        },
        response_body_sink,
    )
    .await?;

    Ok(transaction)
}
