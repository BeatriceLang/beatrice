use chumsky::{
    Parser,
    prelude::{just, select},
};

use crate::{ast::ty::Type, lexing::token::Token};

pub(super) fn array_ty<'a>(ty: parser_type!(Type)) -> parser_type!(Type) {
    just(Token::LeftSquareBracket)
        .ignore_then(ty)
        .then_ignore(just(Token::Semicolon))
        .then(select! { Token::Number(size) => size })
        .then_ignore(just(Token::RightSquareBracket))
        .map(|(element_ty, size)| Type::Array {
            element_ty: Box::new(element_ty),
            size: size.try_into().unwrap(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{test_parse, test_tokens, ty::ty};

    #[test]
    fn parses_array_ty() {
        let tokens = test_tokens![
            Token::LeftSquareBracket,
            Token::I32,
            Token::Semicolon,
            Token::Number(4),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(array_ty(ty()), &tokens),
            Type::Array {
                element_ty: Box::new(Type::I32),
                size: 4,
            }
        );
    }

    #[test]
    fn parses_nested_array_ty() {
        let tokens = test_tokens![
            Token::LeftSquareBracket,
            Token::LeftSquareBracket,
            Token::Bool,
            Token::Semicolon,
            Token::Number(2),
            Token::RightSquareBracket,
            Token::Semicolon,
            Token::Number(3),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(array_ty(ty()), &tokens),
            Type::Array {
                element_ty: Box::new(Type::Array {
                    element_ty: Box::new(Type::Bool),
                    size: 2,
                }),
                size: 3,
            }
        );
    }
}
