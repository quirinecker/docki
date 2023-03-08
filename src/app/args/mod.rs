use clap::Parser;

use self::structure::Args;

mod structure;

pub fn args() -> Args {
    return Args::parse();

}
