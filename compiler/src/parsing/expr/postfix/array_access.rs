use chumsky::{Parser, primitive::just};

use crate::{
    ast::expression::Expression,
    lexing::token::Token,
    parsing::expr::postfix::Postfix,
};

pub(super) fn array_access<'a>(expr: parser_type!(Expression)) -> parser_type!(Postfix) {
    just(Token::LeftSquareBracket)
        .ignore_then(expr)
        .then_ignore(just(Token::RightSquareBracket))
        .map(Postfix::ArrayAccess)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        parsing::{expr::expr, test_parse, test_tokens},
    };

    #[test]
    fn parses_array_access_expr() {
        let tokens = test_tokens![
            Token::LeftSquareBracket,
            Token::Number(0),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(array_access(expr()), &tokens),
            Postfix::ArrayAccess(Expression::Number(0))
        );
    }

    #[test]
    fn parses_array_access_expr_with_expr_index() {
        let tokens = test_tokens![
            Token::LeftSquareBracket,
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(array_access(expr()), &tokens),
            Postfix::ArrayAccess(
                Expression::BinaryOp {
                    lhs: Expression::Number(1).into(),
                    kind: BinaryOpKind::Add,
                    rhs: Expression::Number(2).into(),
                }
            )
        );
    }
}
