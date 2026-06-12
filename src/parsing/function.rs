use chumsky::{IterParser, Parser, prelude::just};

use crate::{
    ast::Function,
    lexing::token::Token,
    parsing::{block::block, ident::ident, ty::ty},
};

pub fn function<'a>() -> parser_type!(Function) {
    just(Token::Fn)
        .ignore_then(ident())
        .then_ignore(just(Token::LeftParen))
        .then(
            ident()
                .then_ignore(just(Token::Colon))
                .then(ty())
                .separated_by(just(Token::Comma))
                .collect(),
        )
        .then_ignore(just(Token::RightParen))
        .then_ignore(just(Token::RetArrow))
        .then(ty())
        .then(block())
        .map(|(((name, params), return_type), body)| Function {
            name,
            params,
            return_type,
            body,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_function() {
        use chumsky::Parser as _;

        let tokens = [
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
            function().parse(&tokens).unwrap(),
            Function {
                name: "main".into(),
                params: vec![],
                return_type: crate::ast::Type::I32,
                body: crate::ast::Block {
                    statements: vec![crate::ast::statement::Statement::Return(
                        crate::ast::expression::Expression::Number(42)
                    )],
                },
            }
        );
    }
}
