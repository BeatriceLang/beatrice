use chumsky::{IterParser, Parser, prelude::end};

use crate::{ast::Program, parsing::item::item};

pub fn program<'a>() -> parser_type!(Program) {
    item()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|items| Program { items })
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
                items: vec![crate::ast::item::Item::Function(crate::ast::item::Function {
                    name: test_ident("main"),
                    params: vec![],
                    return_type: Some(crate::ast::ty::Type::I32),
                    body: crate::ast::Block {
                        statements: vec![crate::ast::statement::Statement::Return(Some(
                            crate::ast::expression::Expression::Number(42)
                        ))],
                    },
                })],
            }
        );
    }
}
