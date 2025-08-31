use base64::prelude::*;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

mod common;

#[tokio::test]
async fn test_basic_auth() {
    let mock_server = MockServer::start().await;
    let user_and_pass = "user:pass";

    Mock::given(method("GET"))
        .and(path("/basic"))
        .and(header(
            "authorization",
            format!("Basic {}", BASE64_STANDARD.encode(user_and_pass)),
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/basic", mock_server.uri());

    common::cli()
        .args(["http", "get", &url, "--auth", "user:pass"])
        .assert()
        .success();
}

#[tokio::test]
async fn test_bearer_auth() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/basic"))
        .and(header("authorization", "Bearer t0k3n"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/basic", mock_server.uri());

    common::cli()
        .args(["http", "get", &url, "--bearer", "t0k3n"])
        .assert()
        .success();
}
