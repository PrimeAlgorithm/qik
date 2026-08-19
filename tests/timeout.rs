use std::time::Duration;

use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

mod common;

#[tokio::test]
async fn test_timeout() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/slow"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(800)))
        .mount(&mock_server)
        .await;

    let url = format!("{}/slow", mock_server.uri());

    common::cli()
        .args(["http", "get", &url, "--timeout", "100ms"])
        .assert()
        .code(4);
}
