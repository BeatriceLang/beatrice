use chumsky::{IterParser, Parser, prelude::end};

use crate::{
    ast::Program,
    parsing::{BeatriceParser, function::function},
};

pub fn program<'a>() -> BeatriceParser<'a, Program> {
    function()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| Program { functions })
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_program() {
        use chumsky::Parser as _;

        let tokens = [
            crate::lexing::token::Token::Fn,
            crate::lexing::token::Token::Ident("main".into()),
            crate::lexing::token::Token::LeftParen,
            crate::lexing::token::Token::RightParen,
            crate::lexing::token::Token::RetArrow,
            crate::lexing::token::Token::I32,
            crate::lexing::token::Token::LeftBrace,
            crate::lexing::token::Token::Return,
            crate::lexing::token::Token::Number(42),
            crate::lexing::token::Token::Semicolon,
            crate::lexing::token::Token::RightBrace,
        ];

        assert_eq!(
            program().parse(&tokens).unwrap(),
            Program {
                functions: vec![crate::ast::Function {
                    name: "main".into(),
                    return_type: crate::ast::Type::I32,
                    body: crate::ast::Block {
                        statements: vec![crate::ast::statement::Statement::Return(
                            crate::ast::expression::Expression::Number(42)
                        )],
                    },
                }],
            }
        );
    }
}
