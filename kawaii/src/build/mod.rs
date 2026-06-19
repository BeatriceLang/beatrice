use std::{env::current_dir, path::PathBuf};

use anyhow::{Context, Result};

use crate::project_info::ProjectInfo;

mod collect;
mod compile;
mod link;

struct KawaiiBuild {
    state: KawaiiBuildState,
    project: ProjectInfo,
}

enum KawaiiBuildState {
    Collect,
    Compile { sources: Vec<PathBuf> },
    Link { objects: Vec<PathBuf> },
}

impl KawaiiBuild {
    pub const fn new(project: ProjectInfo) -> Self {
        Self {
            state: KawaiiBuildState::Collect,
            project,
        }
    }

    fn advance_to(&mut self, state: KawaiiBuildState) {
        self.state = state;
    }
}

pub fn build(project: ProjectInfo) -> Result<()> {
    let mut kawaii_build = KawaiiBuild::new(project.clone());

    kawaii_build
        .collect()
        .context("Failed to collect source files")?;
    kawaii_build.compile().context("Failed to compile")?;
    kawaii_build.link().context("Failed to link")?;

    eprintln!(
        "Built kawaii project `{}` (Artifact at `{}`)",
        project.name.clone(),
        current_dir()?.join("target").join(project.name).display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{KawaiiBuild, KawaiiBuildState};
    use crate::test_utils::project;

    #[test]
    fn starts_in_collect_state() {
        let build = KawaiiBuild::new(project());

        assert!(matches!(build.state, KawaiiBuildState::Collect));
    }
}
