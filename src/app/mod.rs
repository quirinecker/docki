mod commands;
pub mod build;
pub mod fs_util;
pub mod watcher;
pub mod log;
pub mod config;

use std::env;

use crate::app::config::config::Config;

use self::config::{args, arguments::CommandArg};
use self::commands::build::build;
use self::commands::completions::completions;
use self::commands::health::health;
use self::commands::install_reveal::install_reveal;
use self::commands::serve::serve;

pub struct App;

impl App {

    pub async fn start(&self) {
        let args = args();
		let config = Config::load().unwrap_or(Config::default()).merge_with_args(&args);
        Self::setup_environment_variables();

        match args.command {
            CommandArg::Build { .. } => build(&config).await,
            CommandArg::Health => health(),
            CommandArg::InstallReveal => install_reveal().await,
            CommandArg::Serve { .. } => serve(&config).await,
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


