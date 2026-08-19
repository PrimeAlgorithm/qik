mod common;

use predicates::prelude::predicate;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn body_output_contains_only_the_response_body() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/body"))
        .respond_with(ResponseTemplate::new(200).set_body_string("hello"))
        .mount(&mock_server)
        .await;

    common::cli()
        .args([
            "http",
            "get",
            &format!("{}/body", mock_server.uri()),
            "--output",
            "body",
        ])
        .assert()
        .success()
        .stdout("hello");
}

#[tokio::test]
async fn check_status_fails_after_printing_the_response() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_string("missing"))
        .mount(&mock_server)
        .await;

    common::cli()
        .args([
            "--no-color",
            "http",
            "get",
            &format!("{}/missing", mock_server.uri()),
            "--output",
            "response",
            "--check-status",
        ])
        .assert()
        .code(6)
        .stdout(predicate::str::contains("404 Not Found"))
        .stdout(predicate::str::contains("missing"))
        .stderr(predicate::str::contains(
            "server returned HTTP 404 Not Found",
        ));
}

#[tokio::test]
async fn malformed_json_response_is_still_printed() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/broken"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string("not json"),
        )
        .mount(&mock_server)
        .await;

    common::cli()
        .args([
            "--no-color",
            "http",
            "get",
            &format!("{}/broken", mock_server.uri()),
            "--output",
            "response",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("not json"));
}

#[tokio::test]
async fn response_size_limit_is_enforced() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/large"))
        .respond_with(ResponseTemplate::new(200).set_body_string("too large"))
        .mount(&mock_server)
        .await;

    common::cli()
        .args([
            "http",
            "get",
            &format!("{}/large", mock_server.uri()),
            "--max-response-size",
            "4B",
        ])
        .assert()
        .code(8)
        .stderr(predicate::str::contains(
            "response body exceeds the configured limit of 4 bytes",
        ));
}

#[tokio::test]
async fn additional_sensitive_headers_can_be_redacted() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/secret"))
        .respond_with(
            ResponseTemplate::new(200).insert_header("x-api-key", "super-secret"),
        )
        .mount(&mock_server)
        .await;

    common::cli()
        .args([
            "http",
            "get",
            &format!("{}/secret", mock_server.uri()),
            "--redact-header",
            "x-api-key",
            "--output",
            "response",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("<redacted>"))
        .stdout(predicate::str::contains("super-secret").not());
}

#[tokio::test]
async fn unreadable_client_identity_is_an_error() {
    common::cli()
        .args([
            "http",
            "get",
            "https://example.invalid",
            "--identity-pem",
            "/definitely/missing/qik-identity.pem",
        ])
        .assert()
        .code(5)
        .stderr(predicate::str::contains(
            "failed to read PEM identity from /definitely/missing/qik-identity.pem",
        ));
}

#[tokio::test]
async fn body_output_streams_responses_larger_than_the_formatting_limit() {
    const BODY_SIZE: usize = 11 * 1024 * 1024;

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/download"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![b'x'; BODY_SIZE]))
        .mount(&mock_server)
        .await;

    let output = common::cli()
        .args([
            "http",
            "get",
            &format!("{}/download", mock_server.uri()),
            "--output",
            "body",
        ])
        .output()
        .expect("qik should execute");

    assert!(output.status.success());
    assert_eq!(output.stdout.len(), BODY_SIZE);
    assert!(output.stdout.iter().all(|byte| *byte == b'x'));
}

#[test]
fn transport_failures_have_a_stable_exit_code() {
    common::cli()
        .args([
            "http",
            "get",
            "http://127.0.0.1:9",
            "--connect-timeout",
            "100ms",
        ])
        .assert()
        .code(3);
}
