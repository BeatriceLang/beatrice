use anyhow::Result;
use xshell::{Shell, cmd};

use crate::build::{KawaiiBuild, KawaiiBuildState};
use crate::project_layout::ProjectLayout;

impl KawaiiBuild {
    pub(super) fn link(&self) -> Result<()> {
        let KawaiiBuildState::Link { objects } = &self.state else {
            panic!("Unexpected kawaii build state")
        };

        let linker = self.linker();
        let output = self.output()?;

        let sh = Shell::new()?;
        cmd!(sh, "{linker} {objects...} -o {output}").run()?;

        Ok(())
    }

    fn output(&self) -> Result<std::path::PathBuf> {
        Ok(ProjectLayout::current()?.artifact(&self.project))
    }

    const fn linker(&self) -> &str {
        if self.project.freestanding {
            "ld"
        } else {
            "cc"
        }
    }
}
