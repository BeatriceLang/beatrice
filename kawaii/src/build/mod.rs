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
pub(super) mod test_support {
    use std::{
        env, fs,
        path::{Path, PathBuf},
        panic::{RefUnwindSafe, UnwindSafe, catch_unwind, resume_unwind},
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::project_info::ProjectInfo;

    static CURRENT_DIR_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    pub(super) fn temp_test_dir() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = env::temp_dir().join(format!("kawaii-unit-test-{}-{suffix}", std::process::id()));

        fs::create_dir(&dir).unwrap();

        dir
    }

    pub(super) fn with_current_dir<T>(
        dir: &Path,
        test: impl FnOnce() -> T + UnwindSafe + RefUnwindSafe,
    ) -> T {
        let _guard = CURRENT_DIR_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|err| err.into_inner());
        let previous_dir = env::current_dir().unwrap();
        env::set_current_dir(dir).unwrap();

        let result = catch_unwind(test);

        env::set_current_dir(previous_dir).unwrap();

        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    pub(super) fn project() -> ProjectInfo {
        ProjectInfo {
            name: "test".to_string(),
            freestanding: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KawaiiBuild, KawaiiBuildState, test_support::project};

    #[test]
    fn starts_in_collect_state() {
        let build = KawaiiBuild::new(project());

        assert!(matches!(build.state, KawaiiBuildState::Collect));
    }
}
