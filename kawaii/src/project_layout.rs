use std::{env::current_dir, path::PathBuf};

use anyhow::Result;

use crate::project_info::ProjectInfo;

pub struct ProjectLayout {
    root: PathBuf,
}

impl ProjectLayout {
    pub fn current() -> Result<Self> {
        Ok(Self {
            root: current_dir()?,
        })
    }

    pub fn src_dir(&self) -> PathBuf {
        self.root.join("src")
    }

    pub fn target_dir(&self) -> PathBuf {
        self.root.join("target")
    }

    pub fn objects_dir(&self) -> PathBuf {
        self.target_dir().join("objects")
    }

    pub fn artifact(&self, project: &ProjectInfo) -> PathBuf {
        self.target_dir().join(&project.name)
    }
}
