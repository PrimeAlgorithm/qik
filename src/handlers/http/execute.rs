use reqwest::{Client, Method};

use crate::commands::http::CommonHttpArgs;

pub struct RequestInformation<'a> {
    pub method: Method,

    pub common: &'a CommonHttpArgs,
}

pub async fn execute(client: &Client, req_info: RequestInformation<'_>) {
    println!("Method: {}", req_info.method);
    println!("Headers: {:?}", req_info.common.headers);

    let mut request = client.request(req_info.method, "");

    if let Some(headers) = &req_info.common.headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }
}
