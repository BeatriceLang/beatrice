use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn deref<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    just(Token::Multiply)
        .ignore_then(expr)
        .map(|ptr| Expression::Deref { ptr: Box::new(ptr) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::Expression,
        parsing::{expr::expr, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_deref_expr() {
        let tokens = test_tokens![Token::Multiply, Token::Ident("ptr".into())];

        assert_eq!(
            test_parse(deref(expr()), &tokens),
            Expression::Deref {
                ptr: Expression::Ident(test_ident("ptr")).into(),
            }
        );
    }

    #[test]
    fn parses_nested_deref_expr() {
        let tokens = test_tokens![Token::Multiply, Token::Multiply, Token::Ident("ptr".into()),];

        assert_eq!(
            test_parse(deref(expr()), &tokens),
            Expression::Deref {
                ptr: Expression::Deref {
                    ptr: Expression::Ident(test_ident("ptr")).into(),
                }
                .into(),
            }
        );
    }
}
