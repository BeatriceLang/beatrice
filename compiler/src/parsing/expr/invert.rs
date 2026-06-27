use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn invert<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    just(Token::ExclamationMark)
        .ignore_then(expr)
        .map(|val| Expression::Invert(Box::new(val)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::Expression,
        parsing::{expr::expr, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_invert_expr() {
        let tokens = test_tokens![Token::ExclamationMark, Token::Ident("value".into())];

        assert_eq!(
            test_parse(invert(expr()), &tokens),
            Expression::Invert(Expression::Ident(test_ident("value")).into())
        );
    }

    #[test]
    fn parses_nested_invert_expr() {
        let tokens = test_tokens![
            Token::ExclamationMark,
            Token::ExclamationMark,
            Token::Ident("value".into()),
        ];

        assert_eq!(
            test_parse(invert(expr()), &tokens),
            Expression::Invert(
                Expression::Invert(Expression::Ident(test_ident("value")).into()).into()
            )
        );
    }
}
