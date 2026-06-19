use std::fs;

use anyhow::{Context, Result};
use clap::Parser;

use crate::{
    args::{Args, Command},
    build::build,
    new::new,
    project_info::ProjectInfo,
};

mod args;
mod build;
mod new;
mod project_info;

fn main() -> Result<()> {
    let arg = Args::parse();

    match arg.command {
        Command::Build => build(ProjectInfo::from_project_toml()?)?,
        Command::New(args) => new(args)?,
    }

    Ok(())
}
