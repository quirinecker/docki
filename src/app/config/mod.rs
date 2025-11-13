use clap::Parser;

use self::arguments::Args;

pub mod arguments;
pub mod config;

pub fn args() -> Args {
    return Args::parse();
}
