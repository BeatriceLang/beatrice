use anyhow::Result;
use clap::Parser;

use crate::args::{Args, Command};

mod args;
mod build;

fn main() -> Result<()> {
    let arg = Args::parse();

    match arg.command {
        Command::Build => build::run()?,
    }

    Ok(())
}
