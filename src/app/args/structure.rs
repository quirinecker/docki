use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Option<CommandArg>
}

#[derive(Subcommand)]
enum CommandArg {
    Build,
}
