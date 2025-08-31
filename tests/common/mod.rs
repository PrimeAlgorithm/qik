use assert_cmd::Command;

pub fn cli() -> Command {
    Command::cargo_bin("qik").expect("binary name 'qik' must match your Cargo.toml")
}
