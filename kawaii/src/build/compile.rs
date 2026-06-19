use std::path::{self, PathBuf};

use xshell::{Shell, cmd};

use crate::build::{KawaiiBuild, KawaiiBuildState};

impl KawaiiBuild {
    pub(super) fn compile(&mut self) {
        let KawaiiBuildState::Compile { sources } = &self.state else {
            panic!("Unexpected kawaii build state");
        };
    }
}
