use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ident::ident};

pub(super) fn field_access<'a>() -> parser_type!(Expression) {
    ident()
        .then_ignore(just(Token::Dot))
        .then(ident())
        .map(|(base, field)| Expression::FieldAccess { base, field })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::Expression,
        parsing::{test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_field_access() {
        let tokens = test_tokens![
            Token::Ident("point".into()),
            Token::Dot,
            Token::Ident("x".into()),
        ];

        assert_eq!(
            test_parse(field_access(), &tokens),
            Expression::FieldAccess {
                base: test_ident("point"),
                field: test_ident("x"),
            }
        );
    }
}
