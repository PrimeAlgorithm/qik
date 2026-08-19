# Qik

Qik is an asynchronous HTTP command-line client written in Rust. It is designed for two common jobs:

- interactively inspecting an HTTP request and its response;
- safely piping or downloading the raw response body in scripts.

It supports JSON, XML, forms, file uploads, authentication, cookies, proxies, custom certificates, mTLS, timeouts, response limits, and stable exit codes.

> Qik is a focused developer and operations tool, not a complete replacement for curl. See [Current limitations](#current-limitations) before adopting it in critical automation.

## Documentation map

- [Quick start](#quick-start): build Qik and make the first request.
- [Common recipes](#common-recipes): payloads, authentication, uploads, scripts, and downloads.
- [Operational controls](#operational-controls): timeouts, limits, proxies, and TLS.
- [Sensitive output](#sensitive-output): what Qik redacts and what it does not.
- [Exit codes](#exit-codes): the contract for automation.
- [Architecture](docs/ARCHITECTURE.md): the internal request and response lifecycle.
- [Contributing](CONTRIBUTING.md): development setup and validation.

## Quick start

### Build

Qik requires Rust 1.88 or newer.

```bash
git clone https://github.com/albuilds/qik.git
cd qik
cargo build --release
```

The resulting binary is `target/release/qik`. To install it in Cargo's binary directory:

```bash
cargo install --path .
```

### Make a request

Every HTTP operation follows the same shape:

```text
qik http <verb> <URL> [options]
```

```bash
qik http get https://api.example.com/users/42
```

By default, Qik prints a readable representation of both sides of the transaction:

```text
Request:
GET https://api.example.com/users/42 HTTP/1.1 (auto-negotiated)
host: api.example.com

<no body>

Response:
HTTP/1.1 200 OK
content-type: application/json

{
  "id": 42,
  "name": "Ada"
}
```

Supported verbs are `get`, `post`, `put`, `delete`, `patch`, `head`, and `options`.

## Common recipes

### Query parameters and headers

Options such as `--param`, `--header`, `--cookie`, and `--form` are repeatable.

```bash
qik http get https://api.example.com/search \
  --param q=rust \
  --param page=2 \
  --header "Accept: application/json"
```

### JSON, XML, and raw bodies

Qik validates JSON and XML before sending them and supplies the corresponding `Content-Type` unless you override it.

```bash
qik http post https://api.example.com/users \
  --json '{"name":"Ada","role":"admin"}'

qik http post https://api.example.com/events \
  --xml '<event><name>deploy</name></event>'

qik http post https://api.example.com/echo \
  --raw 'unmodified body'
```

Only one payload type may be supplied per request.

### Forms and file uploads

Text-only fields use `application/x-www-form-urlencoded`:

```bash
qik http post https://api.example.com/login \
  --form username=ada \
  --form remember=true
```

If any field references a file, Qik switches to multipart encoding:

```bash
qik http post https://api.example.com/uploads \
  --form 'file=@./report.pdf;filename=quarterly-report.pdf' \
  --form description='Quarterly report'
```

### Authentication and cookies

```bash
# Basic authentication
qik http get https://api.example.com/private --auth 'user:password'

# Bearer authentication
qik http get https://api.example.com/private --bearer "$TOKEN"

# Multiple cookies
qik http get https://api.example.com/dashboard \
  --cookie session=abc123 \
  --cookie theme=dark
```

Basic and Bearer authentication cannot be used together.

### Use the response in a script

Select what Qik writes with `--output`:

| Mode | Output | Typical use |
| --- | --- | --- |
| `all` | Request and response | Interactive debugging; default |
| `request` | Request only | Verify request construction |
| `response` | Status, headers, formatted body | Logs and response inspection |
| `body` | Original response bytes | Pipelines and downloads |

Every mode performs the HTTP request. `request` changes what is displayed; it is not a dry-run mode.

```bash
qik http get https://api.example.com/users --output body | jq '.[].name'
```

Body-only output is streamed directly to stdout. It is not colorized, reformatted, buffered in full, or given an extra newline, so binary downloads are safe:

```bash
qik http get https://downloads.example.com/archive.tar.gz \
  --output body > archive.tar.gz
```

For an important download, write to a temporary path and rename it only after success:

```bash
qik http get "$URL" --output body --check-status > artifact.tmp &&
  mv artifact.tmp artifact.tar.gz
```

### Fail a script on HTTP errors

An HTTP 404 or 500 is still a completed HTTP transaction, so it exits successfully by default. Add `--check-status` to return exit code 6 for 4xx and 5xx responses:

```bash
qik http get https://api.example.com/users/42 \
  --output response \
  --check-status
```

The selected output is written before Qik returns the failure code.

## Operational controls

### Timeouts and redirects

```bash
qik http get https://api.example.com/slow \
  --connect-timeout 3s \
  --timeout 15s \
  --redirects 3
```

- `--connect-timeout` limits connection establishment and defaults to `10s`.
- `--timeout` limits the complete request and defaults to `30s`.
- `--redirects N` sets the redirect limit; `0` disables redirects.

Durations accept values supported by `humantime`, including `500ms`, `15s`, and `2m`.

### Response-size limits

Formatted modes buffer the response so it can be displayed and pretty-printed. They therefore have a default 10 MiB limit. Body-only mode streams and has no default size limit.

Use `--max-response-size` to set an explicit limit for either path:

```bash
qik http get https://api.example.com/export \
  --output body \
  --max-response-size 100MiB
```

Accepted units are `B`, `KiB`, `MiB`, and `GiB`; the shorter `KB`, `MB`, and `GB` forms are also accepted. A value of `0` explicitly disables the limit.

If a streaming server omits `Content-Length`, Qik may have written a partial body before a timeout, transport failure, or size violation becomes known.

### Proxy and HTTP version

```bash
qik http get https://api.example.com \
  --proxy http://127.0.0.1:8080 \
  --http-version 2
```

`--http-version` accepts `auto`, `1.0`, `1.1`, or `2`. The default, `auto`, lets Reqwest negotiate the protocol.

### TLS and client certificates

Add one or more private CA roots:

```bash
qik http get https://internal.example.com --cacert ./company-ca.pem
```

Use one of the supported mTLS identity forms:

```bash
# Combined PEM identity
qik http get https://internal.example.com --identity-pem ./identity.pem

# PEM certificate chain and PKCS#8 private key
qik http get https://internal.example.com \
  --cert ./client-cert.pem \
  --key ./client-key.pem

# PKCS#12/PFX archive
qik http get https://internal.example.com \
  --p12 ./client.p12 \
  --p12-pass "$P12_PASSWORD"
```

Unreadable or invalid identity files fail explicitly rather than allowing the request to continue without client authentication.

The following options disable TLS protections and should only be used temporarily on a trusted network:

- `--insecure`: accept invalid certificates;
- `--no-verify-hostname`: accept a hostname mismatch.

Qik prints a warning whenever either option is active.

## Sensitive output

Qik prints network data to the terminal; treat that output as sensitive.

The following headers are automatically redacted:

- `Authorization` and `Proxy-Authorization` retain the scheme but hide the credential;
- `Cookie` and `Set-Cookie` are fully hidden.

Add application-specific secret headers with repeatable `--redact-header` options:

```bash
qik http get https://api.example.com \
  --header "X-Api-Key: $API_KEY" \
  --redact-header x-api-key
```

Redaction applies to displayed request and response headers. It does **not** automatically protect URLs, query parameters, bodies, or arbitrary headers. Use `--output response` or `--output body` when the request itself contains sensitive information.

Colors are disabled when stdout is not a terminal, when `--no-color` is used, or when the standard `NO_COLOR` environment variable is nonempty.

## Exit codes

| Code | Meaning |
| ---: | --- |
| `0` | Success, including HTTP errors unless `--check-status` is used |
| `1` | Unclassified application or formatting failure |
| `2` | Invalid command-line usage |
| `3` | Network or transport failure |
| `4` | Connection or request timeout |
| `5` | TLS, CA certificate, or client-identity failure |
| `6` | HTTP 4xx/5xx response with `--check-status` |
| `7` | Output or write failure |
| `8` | Response exceeded the configured or formatted-output limit |

Example handling:

```bash
qik http get "$URL" --output body --check-status > response.tmp
status=$?

case "$status" in
  0) mv response.tmp response.bin ;;
  3) echo "network failure" >&2 ;;
  4) echo "request timed out" >&2 ;;
  5) echo "TLS failure" >&2 ;;
  6) echo "server returned an HTTP error" >&2 ;;
  7) echo "output could not be written" >&2 ;;
  8) echo "response exceeded its limit" >&2 ;;
  *) echo "qik failed with exit code $status" >&2 ;;
esac
```

## Command overview

Run `qik --help`, `qik http --help`, or `qik http <verb> --help` for the authoritative generated reference.

| Option | Purpose |
| --- | --- |
| `--header "Name: Value"` | Add a request header; repeatable |
| `--param key=value` | Add a query parameter; repeatable |
| `--cookie name=value` | Add an in-memory cookie; repeatable |
| `--auth user:pass` | Use Basic authentication |
| `--bearer TOKEN` | Use Bearer authentication |
| `--raw TEXT` | Send an untyped literal body |
| `--json JSON` | Validate and send JSON |
| `--xml XML` | Validate and send XML |
| `--form key=value` | Add a text or file form field; repeatable |
| `--output MODE` | Select `all`, `request`, `response`, or `body` |
| `--check-status` | Treat HTTP 4xx/5xx as exit code 6 |
| `--no-color` | Disable ANSI styling |
| `--timeout DURATION` | Set the complete request timeout |
| `--connect-timeout DURATION` | Set the connection timeout |
| `--max-response-size SIZE` | Bound accepted response bytes |
| `--redirects N` | Set the redirect limit |
| `--proxy URL` | Route through an HTTP or SOCKS proxy |
| `--http-version VERSION` | Select `auto`, `1.0`, `1.1`, or `2` |
| `--cacert PATH` | Add a trusted CA certificate or PEM bundle |
| `--redact-header NAME` | Redact an additional displayed header; repeatable |

## How Qik is organized

At a high level, Qik processes a command in four stages:

```text
CLI parsing → request construction → HTTP execution → output formatting/streaming
```

- `src/commands/` defines HTTP verbs, options, and value parsers.
- `src/handlers/http/requests/` builds the client and request, executes it, and reads or streams the response.
- `src/models/` holds the request/response transaction types used by formatted output.
- `src/output/` renders headers, status lines, and bodies and writes them to the selected destination.
- `src/error.rs` assigns stable runtime error categories and exit codes.
- `tests/` contains end-to-end tests backed by local mock HTTP servers.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the complete request lifecycle and design tradeoffs.

## Current limitations

- Formatted response modes must buffer the complete body, although the buffer is bounded.
- Request bodies are provided inline or through multipart file fields; general `@file` and stdin body input are not implemented.
- Displayed requests are reconstructed from Qik's request model and may omit headers generated internally by Reqwest.
- Multipart request contents are streamed but represented as a placeholder in displayed output.
- Retries are not automatic. Retrying non-idempotent requests requires application-specific policy and is intentionally left to callers.
- Redaction is best effort and is not a substitute for sanitizing logs.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, validation commands, test conventions, and the pull-request checklist.

Qik is distributed under the [MIT License](LICENSE) and is provided without warranty.
