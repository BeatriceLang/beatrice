use std::env::current_dir;

use anyhow::Result;
use xshell::{Shell, cmd};

use crate::{build::build, project_info::ProjectInfo};

pub fn run() -> Result<()> {
    let project_info = ProjectInfo::from_project_toml()?;

    build(project_info.clone())?;

    let output = current_dir()?.join("target").join(project_info.name);

    let sh = Shell::new()?;
    cmd!(sh, "{output}").run()?;

    Ok(())
}
