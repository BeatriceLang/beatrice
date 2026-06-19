use std::{env::current_dir, path::PathBuf};

use anyhow::Result;
use xshell::{Shell, cmd};

use crate::build::{KawaiiBuild, KawaiiBuildState};

impl KawaiiBuild {
    pub(super) fn link(&mut self) -> Result<()> {
        let KawaiiBuildState::Link { objects } = &self.state else {
            panic!("Unexpected kawaii build state")
        };

        let linker = match link_with_crt() {
            true => "cc",
            false => "ld",
        };

        let objects_str: String = objects
            .iter()
            .map(|f| f.to_str().unwrap().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let output = self.output()?;

        let sh = Shell::new()?;
        cmd!(sh, "{linker} {objects_str} -o {output}").run()?;

        Ok(())
    }

    fn output(&self) -> Result<PathBuf> {
        Ok(current_dir()?.join(self.project.name.clone()))
    }
}

// TODO
fn link_with_crt() -> bool {
    true
}
