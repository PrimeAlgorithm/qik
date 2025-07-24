mod cli;
mod commands;
mod handlers;

use crate::cli::execute_cmd;
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    execute_cmd(&cli);
}
