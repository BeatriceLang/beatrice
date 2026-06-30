use chumsky::{IterParser, Parser, prelude::just};

use crate::{
    ast::item::Item,
    lexing::token::Token,
    parsing::{ident::ident, ty::ty},
};

pub fn extern_function<'a>() -> parser_type!(Item) {
    just(Token::Extern)
        .then_ignore(just(Token::Fn))
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
        .then_ignore(just(Token::Semicolon))
        .map(|((name, params), return_type)| Item::ExternFunction {
            name,
            params,
            return_type,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::item::Item;

    #[test]
    fn parses_extern_function() {
        use crate::parsing::{test_ident, test_parse, test_tokens};

        let tokens = test_tokens![
            Token::Extern,
            Token::Fn,
            Token::Ident("puts".into()),
            Token::LeftParen,
            Token::Ident("value".into()),
            Token::Colon,
            Token::String,
            Token::RightParen,
            Token::RetArrow,
            Token::I32,
            Token::Semicolon,
        ];

        assert_eq!(
            test_parse(extern_function(), &tokens),
            Item::ExternFunction {
                name: test_ident("puts"),
                params: vec![(test_ident("value"), crate::ast::ty::Type::String)],
                return_type: Some(crate::ast::ty::Type::I32),
            }
        );
    }

    #[test]
    fn parses_extern_function_without_return_type() {
        use crate::parsing::{test_ident, test_parse, test_tokens};

        let tokens = test_tokens![
            Token::Extern,
            Token::Fn,
            Token::Ident("flush".into()),
            Token::LeftParen,
            Token::RightParen,
            Token::Semicolon,
        ];

        assert_eq!(
            test_parse(extern_function(), &tokens),
            Item::ExternFunction {
                name: test_ident("flush"),
                params: vec![],
                return_type: None,
            }
        );
    }
}
