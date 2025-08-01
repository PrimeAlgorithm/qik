pub mod execute;

use crate::{
    commands::http::{CommonHttpArgs, HttpCommands, PayloadArgs},
    handlers::http::execute::{RequestInformation, execute},
};
use reqwest::{Client, Method};

pub async fn execute_http_command(command: &HttpCommands, client: &Client) {
    println!("Executing HTTP command.");

    match command {
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
    }
}

async fn execute_helper(
    client: &Client,
    req_method: Method,
    common: &CommonHttpArgs,
    body: &Option<PayloadArgs>,
) {
    execute(
        client,
        RequestInformation {
            method: req_method,
            common: common,
            body: body,
        },
    )
    .await
    .unwrap();
}
