use std::path::PathBuf;

use anyhow::{Context, Result};

mod collect;
mod compile;
mod link;

struct KawaiiBuild {
    state: KawaiiBuildState,
}

enum KawaiiBuildState {
    Collect,
    Compile { sources: Vec<PathBuf> },
    Link { objects: Vec<PathBuf> },
}

impl KawaiiBuild {
    pub fn new() -> Self {
        Self {
            state: KawaiiBuildState::Collect,
        }
    }

    fn advance_to(&mut self, state: KawaiiBuildState) {
        self.state = state;
    }
}

pub fn run() -> Result<()> {
    let mut kawaii_build = KawaiiBuild::new();

    kawaii_build
        .collect()
        .context("Failed to collect source files")?;
    kawaii_build.compile().context("Failed to compile")?;
    kawaii_build.link().context("Failed to link")?;

    Ok(())
}
