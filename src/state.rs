use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{ast::Program, diagnostic::Diagnostics, lexing::token::Token, span::Spanned};

pub struct Compiler {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub diagnostics: Diagnostics,
    pub state: CompilerState,
}

pub enum CompilerState {
    Lex(String),
    Parse(Vec<Spanned<Token>>),
    Import(Program),
    Check(Program),
    Codegen(Program),
    Error,
}

impl Compiler {
    pub fn new(source_path: &Path, output_path: PathBuf) -> Result<Self> {
        let source_path = source_path
            .canonicalize()
            .context("Unable to resolve source file path")?;
        let source = fs::read_to_string(&source_path).context("Unable to read source file")?;

        Ok(Self {
            source_path: source_path.clone(),
            diagnostics: Diagnostics::new(source.clone(), source_path),
            state: CompilerState::Lex(source),
            output_path,
        })
    }

    pub fn advance_to(&mut self, state: CompilerState) -> Result<()> {
        self.diagnostics.process()?;
        self.state = state;
        Ok(())
    }
}
