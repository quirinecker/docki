use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: CommandArg
}

#[derive(Subcommand)]
enum CommandArg {
    Build,
    Health,
    InstallReveal
}
