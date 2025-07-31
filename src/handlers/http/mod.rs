pub mod execute;

use crate::{
    commands::http::{CommonHttpArgs, HttpCommands},
    handlers::http::execute::{RequestInformation, execute},
};
use reqwest::{Client, Method};

pub async fn execute_http_command(command: &HttpCommands, client: &Client) {
    println!("Executing HTTP command.");

    match command {
        HttpCommands::Get { common } => execute_helper(client, Method::GET, common).await,
        HttpCommands::Post { common } => execute_helper(client, Method::POST, common).await,
        HttpCommands::Put { common } => execute_helper(client, Method::PUT, common).await,
        HttpCommands::Delete { common } => execute_helper(client, Method::DELETE, common).await,
        HttpCommands::Patch { common } => execute_helper(client, Method::PATCH, common).await,
        HttpCommands::Head { common } => execute_helper(client, Method::HEAD, common).await,
        HttpCommands::Options { common } => execute_helper(client, Method::OPTIONS, common).await,
    }
}

async fn execute_helper(client: &Client, req_method: Method, common: &CommonHttpArgs) {
    execute(
        client,
        RequestInformation {
            method: req_method,
            common: common
        },
    )
    .await
}
