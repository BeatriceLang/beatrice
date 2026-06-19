#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::state::Compiler;

mod ast;
mod check;
mod codegen;
mod diagnostic;
mod import;
mod lexing;
mod parsing;
mod span;
mod state;

/// Compiles a Beatrice source file into an object file.
///
/// # Errors
///
/// Returns an error if the source file cannot be read, diagnostics are emitted,
/// imports cannot be resolved, or object emission fails.
pub fn compile(source_path: &Path, output_path: PathBuf) -> Result<()> {
    let mut compiler = Compiler::new(source_path, output_path)?;

    compiler.lex()?;
    compiler.parse()?;
    compiler.import()?;
    compiler.check()?;
    compiler.codegen()?;

    Ok(())
}
