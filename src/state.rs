use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use crate::{ast::Program, cli_args::Args, diagnostic::Diagnostics, lexing::token::Token};

pub struct Compiler {
    pub output_path: PathBuf,
    pub diagnostics: Diagnostics,
    pub state: CompilerState,
}

pub enum CompilerState {
    Lex(String),
    Parse(Vec<Token>),
    Codegen(Program),
    Error,
}

impl Compiler {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let source_path = args.source_path;
        let source =
            fs::read_to_string(source_path.clone()).context("Unable to read source file")?;

        Ok(Self {
            diagnostics: Diagnostics::new(source.clone(), source_path),
            state: CompilerState::Lex(source),
            output_path: args.output,
        })
    }

    pub fn advance_to(&mut self, state: CompilerState) -> Result<()> {
        self.diagnostics.process()?;
        self.state = state;
        Ok(())
    }
}
