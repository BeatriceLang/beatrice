use std::path::PathBuf;

use crate::build::collect::collect;

mod collect;
mod compile;

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

pub fn run() {
    let mut kawaii_build = KawaiiBuild::new();

    kawaii_build.collect();
}
