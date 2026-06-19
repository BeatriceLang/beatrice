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
    let project_toml =
        fs::read_to_string("Kawaii.toml").context("Failed to read project info (Kawaii.toml)")?;
    let project_info: ProjectInfo =
        toml::from_str(&project_toml).context("Failed to parse project info")?;

    let arg = Args::parse();

    match arg.command {
        Command::Build => build(project_info)?,
        Command::New => new()?,
    }

    Ok(())
}
