# Qik architecture

This document explains how a command becomes an HTTP transaction and why formatted and body-only output follow different response paths.

## Request lifecycle

```text
main
  │
  ├─ parse command line with Clap
  ├─ configure terminal color behavior
  │
  ▼
cli::execute_cmd
  │
  ├─ choose formatted or streaming output
  ▼
handlers::http::requests::execute
  │
  ├─ map the verb to reqwest::Method
  ▼
handlers::http::requests::requests::request
  │
  ├─ load CA roots and the optional client identity
  ├─ build the Reqwest client
  ├─ assemble URL, headers, cookies, authentication, and payload
  ├─ send the request
  ├─ classify transport/TLS/timeout failures
  └─ consume response chunks
       ├─ body mode: write each chunk to the supplied output sink
       └─ formatted mode: append each chunk to a bounded buffer
  │
  ▼
Transaction(RequestSpec, ResponseData)
  │
  ├─ body mode: body was already streamed; only status is inspected
  └─ formatted modes: render the transaction through output::format
```

## CLI and dispatch

`src/cli.rs` owns global output behavior. Clap-specific HTTP arguments live in `src/commands/http/mod.rs`, while custom parsers live under `src/commands/parsers/`.

The CLI chooses whether to pass a `Write` sink into HTTP execution. A sink is supplied only for `--output body`. This decision is made before the response is read, which is what allows body mode to avoid a response-sized allocation.

HTTP verb dispatch is deliberately thin. `src/handlers/http/requests/execute.rs` converts each command variant into a `reqwest::Method` and forwards the shared arguments to the request pipeline.

## Client and request construction

The request pipeline is divided into focused helpers:

- `build_client.rs`: protocol selection, proxy, redirect policy, timeouts, CA roots, and client identity;
- `load_ca_certs.rs`: PEM bundle or DER CA loading;
- `load_client_cert.rs`: combined PEM, certificate/key, or PKCS#12 identity loading;
- `set_headers.rs`, `set_cookies.rs`, and `load_auth.rs`: request metadata;
- `set_payload.rs`: raw, JSON, XML, URL-encoded, and multipart bodies.

Errors loading a requested TLS identity are fatal. Continuing without an identity would change the security behavior requested by the user.

## Response paths

### Formatted output

`all`, `request`, and `response` modes need structured information after execution. Response chunks are therefore accumulated in `BytesMut` and returned in `ResponseData`.

The default formatted-body limit is 10 MiB. It prevents an unexpectedly large response from consuming unbounded memory. `--max-response-size` can replace that limit, including with `0` for explicitly unlimited buffering.

If the response has `Content-Length`, Qik can reject it before reading. Qik still counts received chunks because transfer encoding, decompression, or an incorrect server header can make the actual body differ from the advertised length.

### Body-only output

`--output body` supplies the `Printer` as a `dyn Write` sink. Every response chunk is written immediately and the returned `ResponseData.body` remains empty. This keeps application memory approximately proportional to a network chunk rather than the response.

Body streaming is unlimited by default because downloads are its primary use case. An explicit `--max-response-size` is still enforced. If the limit is discovered after earlier chunks were written, the destination contains a partial response and Qik exits with code 8.

## Formatting and sensitive values

The formatter operates on `RequestSpec` and `ResponseData`:

- JSON-like MIME types are pretty-printed when valid;
- malformed JSON falls back to its original text;
- non-UTF-8 formatted bodies are represented by a byte-count placeholder;
- empty bodies are displayed as `<no body>`;
- status classes receive different terminal colors.

Known authentication and cookie headers are redacted by name. Values selected by `--redact-header` are marked sensitive in the `HeaderMap`, allowing the same formatter to hide matching request and response values.

Raw body output intentionally bypasses formatting and redaction. It must preserve the server's original bytes.

## Error model

Most implementation functions continue to use `anyhow::Result` for context. Errors that need a stable process status are wrapped in `QikError`, which carries an `ErrorKind`.

`main` searches the error chain for `QikError` and uses its assigned exit code. Errors without a runtime category fall back to code 1, while Clap owns command-line usage errors and exits with code 2.

Reqwest identifies timeouts directly. TLS failures originating inside a transport error are recognized from the underlying error-chain messages; locally detected CA and identity failures are categorized directly.

## Testing strategy

Parser and formatter behavior is covered by unit tests next to the implementation. End-to-end behavior lives under `tests/` and uses Wiremock servers so tests do not depend on external services.

Integration tests validate behavior at the process boundary, including stdout, stderr, and exit status. This is particularly important for Qik because its output and exit contract are part of its public interface.

The large-response test sends an 11 MiB response through body mode and verifies every emitted byte. Other tests cover authentication, cookies, forms, redirects, timeouts, redaction, size limits, malformed JSON, and stable exit categories.

## Design boundaries

Qik intentionally does not retry automatically. Safe retry policy depends on the HTTP method, whether a body can be replayed, and application-specific idempotency guarantees.

Formatted request output is a useful representation rather than a packet capture. Reqwest may add or alter wire-level headers, and multipart boundaries are generated while streaming.
