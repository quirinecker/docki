mod commands;
pub mod builder;
pub mod fs_util;
mod args;

use std::env;

use self::args::{args, structure::CommandArg};
use self::commands::build::build;
use self::commands::health::health;
use self::commands::install_reveal::install_reveal;

pub struct App;

impl App {

    pub fn start(&self) {
        let args = args();
        Self::setup_environment_variables();

        match args.command {
            CommandArg::Build => build(),
            CommandArg::Health => health(),
            CommandArg::InstallReveal => install_reveal()
        }
    }

    fn setup_environment_variables() {
        env::set_var("PATH", fs_util::docki_path_env());
    }

    pub fn new() -> Self {
        Self {}
    }

}


