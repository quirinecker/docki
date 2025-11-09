use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandArg,
}

#[derive(Subcommand)]
pub enum ShellArg {
    Bash,
    Fish,
    Zsh,
}

#[derive(Subcommand)]
pub enum CommandArg {
    /// Builds the documentation into a dist folder
    Build {
        #[arg(short, long)]
        offline_reveal: bool,
    },
    /// Checks if everything required for docki is installed
    Health,
    /// Helper command for installing asciidoctor-reveal-js
    InstallReveal,
    /// Starts a Webserver with the live preview of the Documentation
    Serve {
        /// Port for the Live Server
        #[arg(short, long)]
        port: Option<u16>,
    },
    /// Generates completions for the desired shell
    Completions {
        #[command(subcommand)]
        shell: ShellArg,
    },
}
