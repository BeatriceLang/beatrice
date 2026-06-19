use std::path::PathBuf;

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
    pub fn new(project: ProjectInfo) -> Self {
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
    let mut kawaii_build = KawaiiBuild::new(project);

    kawaii_build
        .collect()
        .context("Failed to collect source files")?;
    kawaii_build.compile().context("Failed to compile")?;
    kawaii_build.link().context("Failed to link")?;

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
