mod app;

use std::{fs::File, io::Write};

use app::config::config::Config;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandArg,
}

#[derive(Subcommand)]
pub enum CommandArg {
    /// Generates a default docki.config.toml
    GenerateDefaultConfig,
}

fn main() {
    let args = Args::parse();
    match args.command {
        CommandArg::GenerateDefaultConfig => generate_default_config(),
    }
}

fn generate_default_config() {
    let default_config = Config::default();
    let target_file = "config/docki.config.toml";
	let mut file = File::create(target_file).unwrap();
    let output = toml::to_string_pretty(&default_config).unwrap();

	file.write_all(output.as_bytes()).unwrap();
}
