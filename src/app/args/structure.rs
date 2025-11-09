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
		/// When set to true, docki will download revealjs before building the documentation.
		/// Otherwise it will use the cdn for revealjs
        #[arg(short, long)]
        offline_reveal: bool,
    },
    /// Checks if everything required for docki is installed
    Health,
    /// Deprecated: Helper command for installing asciidoctor-reveal-js
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
