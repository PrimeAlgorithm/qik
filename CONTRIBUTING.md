# Contributing to Qik

Contributions are welcome. Keep changes focused, preserve the command-line compatibility documented in the README, and add process-level tests when behavior visible to users changes.

## Development setup

Qik requires Rust 1.88 or newer.

```bash
git clone https://github.com/albuilds/qik.git
cd qik
cargo build
```

Create a feature branch rather than working directly on `main`:

```bash
git switch -c feat/short-description
```

## Before submitting a change

Run the same checks used by CI:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features
cargo test --all-features
```

For a stricter local Clippy pass:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Build the release binary and smoke-test any affected command:

```bash
cargo build --release
./target/release/qik --help
```

## Tests

- Put parser and formatting unit tests beside the code under `src/`.
- Put CLI behavior in `tests/` and invoke the compiled binary through `tests/common`.
- Use Wiremock for HTTP behavior; tests should not depend on public network services.
- Assert stdout, stderr, and the exact exit code when they are part of the behavior.
- Cover both the successful path and the failure being handled.

When adding a runtime failure category, update all of the following:

1. `ErrorKind` in `src/error.rs`;
2. the exit-code table in `README.md`;
3. an integration test that asserts the numeric code.

## Pull requests

A pull request should explain:

- what user problem it solves;
- any command-line or output changes;
- security or compatibility implications;
- how it was tested.

CI must pass before merging. Avoid including unrelated formatting or refactoring in a behavioral change unless it is necessary for the implementation.
