use anyhow::Result;

use crate::state::{Compiler, CompilerState};

impl Compiler {
    pub fn check(&mut self) -> Result<()> {
        let CompilerState::Check(program) = &self.state else {
            panic!("Unexpected compiler state")
        };

        self.advance_to(CompilerState::Codegen(program.clone()))
    }
}
