use chumsky::Parser;
use logos::Logos;

use crate::{lexing::token::Token, parsing::parser};

mod ast;
mod lexing;
mod parsing;

fn main() {
    let mut lexer = Token::lexer(input());

    let tokens: Vec<Token> = lexer.map(|f| f.clone().unwrap()).collect();

    println!("{:?}", parser().parse(&tokens).unwrap().functions);
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
