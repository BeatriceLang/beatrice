use chumsky::Parser;

use crate::{ast::Program, lexing::token::Token};

mod block;
mod expr;
mod function;
mod ident;
mod program;
mod statement;
mod ty;

macro_rules! parsing_rule {
    {
        $name:ident -> $ret:ty $body:block
    } => {
        pub fn $name<'a>() -> impl chumsky::Parser<'a, &'a [$crate::lexing::token::Token], $ret> $body
    };
}

pub(crate) use parsing_rule;

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> impl Parser<'a, &'a [Token], Program> {
    program::program()
}
