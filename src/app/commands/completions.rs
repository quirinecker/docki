use std::io;

use clap::CommandFactory;
use clap_complete::{generate, shells::{Bash, Fish, Zsh}};

use crate::app::args::structure::{Args, ShellArg};

pub fn completions(shell: ShellArg) {
    let mut command = Args::command();

    match shell {
        ShellArg::Bash => generate(Bash, &mut command, "docki", &mut io::stdout()),
        ShellArg::Fish => generate(Fish, &mut command, "docki", &mut io::stdout()),
        ShellArg::Zsh => generate(Zsh, &mut command, "docki", &mut io::stdout()),
    }
}
