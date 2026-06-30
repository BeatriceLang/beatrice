use chumsky::{IterParser, Parser, prelude::just};

use crate::{
    ast::{ident::Ident, item::Item, ty::Type},
    lexing::token::Token,
    parsing::{block::block, ident::ident, ty::ty},
};

pub fn function<'a>() -> parser_type!(Item) {
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
        .then(just(Token::RetArrow).ignore_then(ty()).or_not())
        .then(block())
        .map(|(((name, params), return_type), body)| Item::Function {
            name,
            params,
            return_type,
            body,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::item::Item;

    #[test]
    fn parses_function() {
        use crate::parsing::{test_ident, test_parse, test_tokens};

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
            test_parse(function(), &tokens),
            Item::Function {
                name: test_ident("main"),
                params: vec![],
                return_type: Some(crate::ast::ty::Type::I32),
                body: crate::ast::Block {
                    statements: vec![crate::ast::statement::Statement::Return(Some(
                        crate::ast::expression::Expression::Number(42)
                    ))],
                },
            }
        );
    }

    #[test]
    fn parses_function_with_params() {
        use crate::parsing::{test_ident, test_parse, test_tokens};

        let tokens = test_tokens![
            Token::Fn,
            Token::Ident("add".into()),
            Token::LeftParen,
            Token::Ident("lhs".into()),
            Token::Colon,
            Token::I32,
            Token::Comma,
            Token::Ident("rhs".into()),
            Token::Colon,
            Token::I32,
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
            test_parse(function(), &tokens),
            Item::Function {
                name: test_ident("add"),
                params: vec![
                    (test_ident("lhs"), crate::ast::ty::Type::I32),
                    (test_ident("rhs"), crate::ast::ty::Type::I32),
                ],
                return_type: Some(crate::ast::ty::Type::I32),
                body: crate::ast::Block {
                    statements: vec![crate::ast::statement::Statement::Return(Some(
                        crate::ast::expression::Expression::Number(42)
                    ))],
                },
            }
        );
    }

    #[test]
    fn parses_function_without_return_type() {
        use crate::parsing::{test_ident, test_parse, test_tokens};

        let tokens = test_tokens![
            Token::Fn,
            Token::Ident("log".into()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(function(), &tokens),
            Item::Function {
                name: test_ident("log"),
                params: vec![],
                return_type: None,
                body: crate::ast::Block { statements: vec![] },
            }
        );
    }
}
