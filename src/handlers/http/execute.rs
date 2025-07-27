use reqwest::{
    Client, Method,
    header::{HeaderName, HeaderValue},
};

pub struct RequestInformation {
    pub method: Method,
    pub headers: Option<Vec<(HeaderName, HeaderValue)>>,
}

pub async fn execute(client: &Client, req_info: RequestInformation) {
    println!("Method: {}", req_info.method);
    println!("Headers: {:?}", req_info.headers);

    let mut request = client.request(req_info.method, "");

    if let Some(headers) = req_info.headers {
        for (key, value) in headers {
            request = request.header(key, value)
        }
    }
}
