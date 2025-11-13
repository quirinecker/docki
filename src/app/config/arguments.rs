use clap::{Parser, Subcommand};
use nu_ansi_term::{AnsiGenericString, Style};

fn github_hyperlink() -> AnsiGenericString<'static, str> {
    return Style::new()
        .bold()
        .underline()
        .paint("https://github.com/quirinecker/docki")
        .hyperlink("https://github.com/quirinecker/docki");
}

#[derive(Parser)]
#[command(after_help = format!("More information like defaults can be found at {}", github_hyperlink()))]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandArg,

    /// The directory where the documentation is located
    #[arg(short, long, global = true)]
    pub input_dir: Option<String>,

    /// The directory where the documentation will be built
    #[arg(short, long, global = true)]
    pub output_dir: Option<String>,
}

#[derive(Subcommand)]
pub enum ShellArg {
    Bash,
    Fish,
    Zsh,
}

#[derive(Subcommand)]
pub enum CommandArg {
    /// Builds the documentation into the specified output_dir
    Build {
        /// When set to true, docki will download revealjs before building the documentation.
        /// Otherwise it will use the cdn for revealjs
        #[arg(long)]
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

impl Args {}
