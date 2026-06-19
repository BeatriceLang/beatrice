use anyhow::Result;
use clap::Parser;

use crate::{
    args::{Args, Command},
    build::build,
    new::new,
    project_info::ProjectInfo,
    run::run,
};

mod args;
mod build;
mod new;
mod project_info;
mod project_layout;
mod run;
#[cfg(test)]
mod test_utils;

fn main() -> Result<()> {
    let arg = Args::parse();

    match arg.command {
        Command::Build => build(&ProjectInfo::from_project_toml()?)?,
        Command::New(args) => new(args)?,
        Command::Run(args) => run(args)?,
    }

    Ok(())
}
