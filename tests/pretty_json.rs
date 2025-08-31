mod common;

use predicates::prelude::predicate;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_pretty_json() {
    let mock_server = MockServer::start().await;

    let body = serde_json::json!({"John": "Doe"});
    Mock::given(method("GET"))
        .and(path("/user_data/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/user_data/1", mock_server.uri());

    common::cli()
        .args(["http", "get", &url])
        .assert()
        .success()
        .stdout(predicate::str::contains("{\n  \"John\": \"Doe\"\n}"));
}
