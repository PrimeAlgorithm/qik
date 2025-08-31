mod common;

use predicates::str::contains;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_redaction() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sensitive"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/sensitive", mock_server.uri());

    common::cli()
        .args([
            "http",
            "get",
            &url,
            "--header",
            "Authorization: Bearer secret-token",
        ])
        .assert()
        .success()
        // Split checks because of colors in output.
        .stdout(contains("authorization"))
        .stdout(contains(": Bearer <redacted>"))
        .stdout(contains("Response:"));
}
