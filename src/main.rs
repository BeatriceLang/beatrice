use logos::Logos;

use crate::lexing::token::Token;

mod lexing;

fn main() {
    let mut lexer = Token::lexer(input());

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(_) => println!("error"),
        }
    }
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
