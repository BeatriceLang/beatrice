use chumsky::Parser;
use inkwell::context::Context;
use logos::Logos;

use crate::{codegen::Codegen, lexing::token::Token, parsing::parser};

mod ast;
mod codegen;
mod lexing;
mod parsing;

fn main() {
    let mut lexer = Token::lexer(input());

    let tokens: Vec<Token> = lexer.map(|f| f.clone().unwrap()).collect();

    let program_ast = parser().parse(&tokens).unwrap();

    let context = Context::create();
    let codegen = Codegen::new(&context, "main", program_ast);

    codegen.generate();
}

// Input code
fn input() -> &'static str {
    const TEST_CODE: &str = "
    fn main() -> i32 {
        return 42;
    }
    ";
    TEST_CODE
}
