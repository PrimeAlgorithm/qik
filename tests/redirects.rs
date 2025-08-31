mod common;

use predicates::prelude::predicate;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_redirects() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/one"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", "/two"))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/two"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&mock_server)
        .await;

    let url = format!("{}/one", mock_server.uri());

    common::cli()
        .args(["http", "get", &url, "--redirects", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));

    common::cli()
        .args(["http", "get", &url, "--redirects", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("302"));
}
