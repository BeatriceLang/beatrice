use chumsky::{
    IterParser, Parser,
    prelude::{choice, end, just},
    primitive::select,
    select,
};

use crate::{
    ast::{Block, Function, Program, Type, expresion::Expression, statement::Statement},
    lexing::token::Token,
    parsing::{expr::expr, function::function, program::program, statement::stmt},
};

mod block;
mod expr;
mod function;
mod ident;
mod program;
mod statement;
mod ty;

#[macro_export]
macro_rules! parsing_rule {
    {
        $name:ident -> $ret:ty $body:block
    } => {
        pub fn $name<'a>() -> impl chumsky::Parser<'a, &'a [$crate::lexing::token::Token], $ret> $body
    };
}

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> impl Parser<'a, &'a [Token], Program> {
    program()
}
