use std::{
    env::current_dir,
    process::{Command, exit},
};

use anyhow::{Result, anyhow};

use crate::{build::build, project_info::ProjectInfo};

#[derive(clap::Args, Debug)]
pub struct RunArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    inner_args: Vec<String>,
}

pub fn run(args: RunArgs) -> Result<()> {
    let project_info = ProjectInfo::from_project_toml()?;

    build(project_info.clone())?;

    let output = current_dir()?.join("target").join(project_info.name);

    exit(
        Command::new(output)
            .args(args.inner_args)
            .status()?
            .code()
            .ok_or_else(|| anyhow!("Failed to get exit code of program"))?,
    );
}
