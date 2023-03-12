use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandArg
}

#[derive(Subcommand)]
pub enum CommandArg {
    /// Builds the documentation into a dist folder
    Build,
    /// Checks if everything required for docki is installed
    Health,
    /// Helper command for installing asciidoctor-reveal-js
    InstallReveal,
    /// Starts a Webserver with the live preview of the Documentation
    Serve {
        /// Port for the Live Server 
        #[arg(short, long)]
        port: Option<u16>
    }
}
