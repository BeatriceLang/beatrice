use anyhow::Result;

use crate::{
    ast::Program,
    diagnostic::Diagnostics,
    state::{Compiler, CompilerState},
};

mod duplicate_function;

impl Compiler {
    pub fn check(&mut self) -> Result<()> {
        let CompilerState::Check(program) = &self.state else {
            panic!("Unexpected compiler state")
        };

        let mut checker = Checker {
            diagnostics: &mut self.diagnostics,
            program,
        };

        checker.run();

        self.advance_to(CompilerState::Codegen(program.clone()))
    }
}

pub struct Checker<'a> {
    diagnostics: &'a mut Diagnostics,
    program: &'a Program,
}

impl<'a> Checker<'a> {
    fn run(&mut self) {
        self.check_duplicate_function();
    }
}
