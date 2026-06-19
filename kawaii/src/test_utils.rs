use std::{
    env, fs,
    panic::{RefUnwindSafe, UnwindSafe, catch_unwind, resume_unwind},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{build::build_options::BuildOptions, project_info::ProjectInfo};

static CURRENT_DIR_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub(crate) fn temp_test_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = env::temp_dir().join(format!("kawaii-unit-test-{}-{suffix}", std::process::id()));

    fs::create_dir(&dir).unwrap();

    dir
}

pub(crate) fn with_current_dir<T>(
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

pub(crate) fn project() -> ProjectInfo {
    ProjectInfo {
        name: "test".to_string(),
        freestanding: false,
        build_options: BuildOptions::default(),
    }
}
