use clap::Parser;

use self::structure::Args;

pub mod structure;

pub fn args() -> Args {
    return Args::parse();

}
