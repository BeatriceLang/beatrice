use std::{fs, path::PathBuf};

use anyhow::Result;
use chumsky::Parser as _;
use clap::Parser as _;
use inkwell::context::Context;
use logos::{Lexer, Logos};

use crate::{
    cli_args::Args,
    codegen::Codegen,
    diagnostic::{Diagnostic, Diagnostics},
    lexing::token::Token,
    parsing::parser,
    state::Compiler,
};

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
