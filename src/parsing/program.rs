use chumsky::{IterParser, Parser, prelude::end};

use crate::{ast::Program, parsing::function::function};

pub fn program<'a>() -> parser_type!(Program) {
    function()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| Program { functions })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_program() {
        use crate::{
            lexing::token::Token,
            parsing::{test_ident, test_parse, test_tokens},
        };

        let tokens = test_tokens![
            Token::Fn,
            Token::Ident("main".into()),
            Token::LeftParen,
            Token::RightParen,
            Token::RetArrow,
            Token::I32,
            Token::LeftBrace,
            Token::Return,
            Token::Number(42),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(program(), &tokens),
            Program {
                functions: vec![crate::ast::Function {
                    name: test_ident("main"),
                    params: vec![],
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
