use std::io::Error;

use clap::CommandFactory;
use clap_complete::{generate_to, shells::{Bash, Zsh, Fish}};

include!("src/app/args/structure.rs");
include!("src/app/fs_util/mod.rs");

fn main() -> Result<(), Error> {
    generate_completions()
}

fn generate_completions() -> Result<(), Error> {
    let mut command = Args::command();
    let home_path = env::var("HOME").expect("could not get home path");
    let out_dir = format!("{}/.docki/completions/", home_path);
    create_dir_recursive(&out_dir);
    generate_to(Bash, &mut command, "docki", &out_dir)?;
    generate_to(Zsh, &mut command, "docki", &out_dir)?;
    generate_to(Fish, &mut command, "docki", &out_dir)?;
    Ok(())
}

