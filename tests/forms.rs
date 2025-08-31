mod common;

use std::io::Write;
use tempfile::NamedTempFile;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, header, header_regex, method, path},
};

#[tokio::test]
async fn test_urlencoded_form() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/form"))
        .and(header("content-type", "application/x-www-form-urlencoded"))
        .and(body_string_contains("a=1"))
        .and(body_string_contains("b=2"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/form", mock_server.uri());

    common::cli()
        .args(["http", "post", &url, "--form", "a=1", "--form", "b=2"])
        .assert()
        .success();
}

#[tokio::test]
async fn test_multipart_form() {
    let mock_server = MockServer::start().await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "Hello World").unwrap();
    let path = tmp.path().to_string_lossy().into_owned();

    Mock::given(method("POST"))
        .and(wiremock::matchers::path("/upload"))
        .and(header_regex("content-type", "multipart/form-data"))
        .and(body_string_contains("name=\"file\""))
        .and(body_string_contains("filename=\"foo.bin\""))
        .and(body_string_contains("name=\"note\""))
        .and(body_string_contains("hi"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let url = format!("{}/upload", mock_server.uri());

    common::cli()
        .args([
            "http",
            "post",
            &url,
            "--form",
            &format!("file=@{};filename=foo.bin", path),
            "--form",
            "note=hi",
        ])
        .assert()
        .success();
}
