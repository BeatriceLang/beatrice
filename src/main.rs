use anyhow::Result;

use crate::state::Compiler;

mod ast;
mod cli_args;
mod codegen;
mod diagnostic;
mod lexing;
mod parsing;
mod state;

fn main() -> Result<()> {
    let mut compiler = Compiler::new()?;

    compiler.lex()?;
    compiler.parse()?;
    compiler.codegen()?;

    Ok(())
}
