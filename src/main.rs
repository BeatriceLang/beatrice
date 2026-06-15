use anyhow::Result;
use clap::Parser;

use crate::{cli_args::Args, state::Compiler};

mod ast;
mod check;
mod cli_args;
mod codegen;
mod diagnostic;
mod lexing;
mod parsing;
mod span;
mod state;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut compiler = Compiler::new(args.source_path, args.output)?;

    compiler.lex()?;
    compiler.parse()?;
    compiler.check()?;
    compiler.codegen()?;

    Ok(())
}
