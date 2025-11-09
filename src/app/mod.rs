mod commands;
pub mod build;
pub mod fs_util;
pub mod watcher;
pub mod log;
mod args;

use std::env;

use self::args::{args, structure::CommandArg};
use self::commands::build::build;
use self::commands::completions::completions;
use self::commands::health::health;
use self::commands::install_reveal::install_reveal;
use self::commands::serve::serve;

pub struct App;

impl App {

    pub async fn start(&self) {
        let args = args();
        Self::setup_environment_variables();

        match args.command {
            CommandArg::Build { offline_reveal } => build(offline_reveal).await,
            CommandArg::Health => health(),
            CommandArg::InstallReveal => install_reveal().await,
            CommandArg::Serve { port } => serve(port).await,
            CommandArg::Completions { shell } => completions(shell)
        };
    }

    fn setup_environment_variables() {
        env::set_var("PATH", fs_util::docki_path_env());
    }

    pub fn new() -> Self {
        Self {}
    }

}


