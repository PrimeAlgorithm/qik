# Qik

Qik is a colorized HTTP CLI built in Rust. 

## Features
Qik supports the following:

* Pretty JSON output (detected via Content-Type)
* Bodies: raw, JSON, XML, urlencoded and multipart 
* Query params, headers, cookies; Basic and Bearer auth
* TLS: custom CA roots and mTLS
* HTTP version selection
* Redirect limit, request timeout, and proxy support


## Installation
### Prerequisites
- Rust 1.56+ 

### Build From Source
1. git clone https://github.com/PrimeAlgorithm/qik
2. cd qik
3. cargo build --release

## Usage
```bash
# GET Request
qik http get https://api.example.com \
  --header "Accept: application/json" \
  --param lang=rust

# POST JSON (Content-Type implied unless you override it with --header)
qik http post https://api.example.com --json '{"name":"John Doe"}'

# Multipart upload (file + text)
qik http post https://api.example.com \
  --form 'file=@"/path/to/file.bin";filename="file.bin"' \
  --form note="hello"

# Basic and Bearer auth
qik http get https://api.example.com --auth "user:pass"
qik http get https://api.example.com --bearer "YOUR_TOKEN"

# Timeouts and redirects
qik http get https://api.example.com --timeout 2s --redirects 3

# Proxy
qik http get https://api.example.com --proxy http://127.0.0.1:8080

# Custom CA
qik http get https://api.example.com --cacert ./company-ca.pem
```

## Example Output
```bash
Request:
GET https://api.example.com HTTP/1.1 (auto-negotiated)
host: api.example.com

<no body>

Response:
HTTP/1.1 200
content-type: application/json

{
  "message": "hello"
}
```

## Command Reference
All verbs can be found under the `http` subcommand. 

```bash
qik http <verb> <URL> [flags] 
```

### Subcommands
`http` Contains all HTTP verbs.

### Common flags:
`--header "Key: Value"` (repeatable)

`--param "key=value"` (repeatable)

`--auth "user:pass"` (Basic auth)

`--bearer TOKEN` (Bearer auth)

`--cookie "name=value"`

`--http-version <auto|1.0|1.1|2>`

`--cacert <PATH>`

`--identity-pem <PATH>`

`--p12 <PATH> --p12-pass <PASS>`

`--timeout <time>`

`--redirects <N>`

`--proxy <URL>`

### Dangerous Flags

`--insecure` Will allow connections to proceed even if the server’s certificate is invalid. 

`--no-verify-hostname` Will allow connections to proceed even if the server’s hostname is invalid.

### Payload Flags

`--raw "<string>"`

`--json '<valid JSON>'`

`--xml '<valid XML>'`

`--form key=value` or `--form key=@/path/file[;filename=name]`

# Security Notes
## Privacy & Redaction
Qik prints request/response details to the terminal. Treat all output as sensitive.

Best-effort redaction is applied to specific headers only:

* `Authorization` and `Proxy-Authorization`: shown as `Scheme <redacted>` (e.g., `Bearer <redacted>`).

* `Cookie` and `Set-Cookie`: shown as `<redacted>`.

Not redacted: request/response bodies (JSON/XML/form), URLs/query params, and any other headers (e.g., X-Api-Key) unless they match the names above.

Redaction is best-effort and not guaranteed. Do not rely on it as a security control. Review and sanitize logs before sharing.

Qik is provided “AS IS” without warranties under the MIT license.

If you notice a redaction issue, please open a security-focused issue or contact the maintainer.

## TLS Verification Bypass (Dangerous)
`--insecure` and `--no-verify-hostname` disable TLS safety checks.

# Project Structure
* `src/`

    * `commands/` command definitions and parsers for those commands

    * `handlers/` handlers for commands (currently only request execution)

    * `models/` data models

    * `output/` formatting (pretty printing) and displaying logic

    * `util`/ small useful helper functions

## Contributing

Pull requests are welcome. 

