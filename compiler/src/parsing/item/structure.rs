use chumsky::{IterParser, Parser, primitive::just};

use crate::{
    ast::Struct,
    lexing::token::Token,
    parsing::{ident::ident, ty::ty},
};

pub(super) fn structure<'a>() -> parser_type!(Struct) {
    let field = ident()
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Comma));
    let body = field
        .repeated()
        .collect()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

    just(Token::Struct)
        .ignore_then(ident())
        .then(body)
        .map(|(name, fields)| Struct { name, fields })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::Type,
        parsing::{test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_empty_struct() {
        let tokens = test_tokens![
            Token::Struct,
            Token::Ident("Empty".into()),
            Token::LeftBrace,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(structure(), &tokens),
            Struct {
                name: test_ident("Empty"),
                fields: vec![],
            }
        );
    }

    #[test]
    fn parses_struct_with_fields() {
        let tokens = test_tokens![
            Token::Struct,
            Token::Ident("Point".into()),
            Token::LeftBrace,
            Token::Ident("x".into()),
            Token::Colon,
            Token::I32,
            Token::Comma,
            Token::Ident("label".into()),
            Token::Colon,
            Token::String,
            Token::Comma,
            Token::Ident("next".into()),
            Token::Colon,
            Token::Multiply,
            Token::I32,
            Token::Comma,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(structure(), &tokens),
            Struct {
                name: test_ident("Point"),
                fields: vec![
                    (test_ident("x"), Type::I32),
                    (test_ident("label"), Type::String),
                    (test_ident("next"), Type::Ptr(Box::new(Type::I32))),
                ],
            }
        );
    }
}
