mod common;

use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

#[tokio::test]
async fn test_cookies() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cookie"))
        .and(header("cookie", "a=1; b=2"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/cookie", mock_server.uri());

    common::cli()
        .args(["http", "get", &url, "--cookie", "a=1", "--cookie", "b=2"])
        .assert()
        .success();
}
