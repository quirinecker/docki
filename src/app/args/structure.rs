use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandArg
}

#[derive(Subcommand)]
pub enum CommandArg {
    Build,
    Health,
    InstallReveal
}
