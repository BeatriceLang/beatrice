use std::{env::current_dir, path::PathBuf};

use anyhow::Result;
use xshell::{Shell, cmd};

use crate::build::{KawaiiBuild, KawaiiBuildState};

impl KawaiiBuild {
    pub(super) fn link(&mut self) -> Result<()> {
        let KawaiiBuildState::Link { objects } = &self.state else {
            panic!("Unexpected kawaii build state")
        };

        let linker = self.linker();
        let output = self.output()?;

        let sh = Shell::new()?;
        cmd!(sh, "{linker} {objects...} -o {output}").run()?;

        Ok(())
    }

    fn output(&self) -> Result<PathBuf> {
        Ok(current_dir()?.join(self.project.name.clone()))
    }

    fn linker(&self) -> &str {
        match self.project.freestanding {
            true => "ld",
            false => "cc",
        }
    }
}
