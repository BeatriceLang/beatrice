use std::fs;

use anyhow::Result;
use chumsky::Parser as _;
use clap::Parser as _;
use inkwell::context::Context;
use logos::Logos;

use crate::{cli_args::Args, codegen::Codegen, lexing::token::Token, parsing::parser};

mod ast;
mod cli_args;
mod codegen;
mod lexing;
mod parsing;

fn main() -> Result<()> {
    let args = Args::parse();

    let input = fs::read_to_string(args.input)?;

    let lexer = Token::lexer(input.as_str());

    let tokens: Vec<Token> = lexer.map(|f| f.clone().unwrap()).collect();

    let program_ast = parser().parse(&tokens).unwrap();

    let context = Context::create();
    let mut codegen = Codegen::new(&context, "main", program_ast);

    codegen.generate();
    codegen.emit_object(&args.output)?;

    Ok(())
}
