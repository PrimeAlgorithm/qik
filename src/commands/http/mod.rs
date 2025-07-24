use clap::Subcommand;

#[derive(Subcommand)]
pub enum HttpCommands {
    Get {},

    Post {},

    Put {},

    Delete {},

    Patch {},

    Head {},

    Options {},
}
