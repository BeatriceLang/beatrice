#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use anyhow::Result;
use clap::Parser;

use crate::{cli_args::Args, state::Compiler};

mod ast;
mod check;
mod cli_args;
mod codegen;
mod diagnostic;
mod import;
mod lexing;
mod parsing;
mod span;
mod state;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut compiler = Compiler::new(args.source_path, args.output)?;

    compiler.lex()?;
    compiler.parse()?;
    compiler.import()?;
    compiler.check()?;
    compiler.codegen()?;

    Ok(())
}
