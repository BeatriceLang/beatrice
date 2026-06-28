use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn addr_of_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    just(Token::AddressOf)
        .ignore_then(expr)
        .map(|value| Expression::AddressOf {
            value: Box::new(value),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::Expression,
        parsing::{expr::expr, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_addr_of_expr() {
        let tokens = test_tokens![Token::AddressOf, Token::Ident("value".into())];

        assert_eq!(
            test_parse(addr_of_expr(expr()), &tokens),
            Expression::AddressOf {
                value: Expression::Ident(test_ident("value")).into(),
            }
        );
    }

    #[test]
    fn parses_nested_addr_of_expr() {
        let tokens = test_tokens![
            Token::AddressOf,
            Token::AddressOf,
            Token::Ident("value".into()),
        ];

        assert_eq!(
            test_parse(addr_of_expr(expr()), &tokens),
            Expression::AddressOf {
                value: Expression::AddressOf {
                    value: Expression::Ident(test_ident("value")).into(),
                }
                .into(),
            }
        );
    }
}
