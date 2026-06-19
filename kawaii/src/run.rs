use std::{
    env::current_dir,
    process::{Command, exit},
};

use anyhow::{Result, anyhow};
use xshell::{Shell, cmd};

use crate::{build::build, project_info::ProjectInfo};

pub fn run() -> Result<()> {
    let project_info = ProjectInfo::from_project_toml()?;

    build(project_info.clone())?;

    let output = current_dir()?.join("target").join(project_info.name);

    exit(
        Command::new(output)
            .status()?
            .code()
            .ok_or(anyhow!("Failed to get exit code of program"))?,
    );
}
