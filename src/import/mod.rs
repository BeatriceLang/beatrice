use anyhow::Result;

use crate::state::{Compiler, CompilerState};

impl Compiler {
    pub fn import(&mut self) -> Result<()> {
        let CompilerState::Import(program) = &self.state else {
            panic!("Unexpected compiler state")
        };

        self.advance_to(CompilerState::Check(program.clone()))
    }
}
