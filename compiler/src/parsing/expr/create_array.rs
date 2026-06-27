use chumsky::{IterParser, Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn create_array<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let elements = expr.separated_by(just(Token::Comma)).collect::<Vec<_>>();

    just(Token::LeftSquareBracket)
        .ignore_then(elements)
        .then_ignore(just(Token::RightSquareBracket))
        .map(|elements| Expression::CreateArray(elements.into_iter().map(Box::new).collect()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        parsing::{expr::expr, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_empty_array_expression() {
        let tokens = test_tokens![Token::LeftSquareBracket, Token::RightSquareBracket];

        assert_eq!(
            test_parse(create_array(expr()), &tokens),
            Expression::CreateArray(vec![])
        );
    }

    #[test]
    fn parses_array_expression_with_single_element() {
        let tokens = test_tokens![
            Token::LeftSquareBracket,
            Token::Number(42),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(create_array(expr()), &tokens),
            Expression::CreateArray(vec![Expression::Number(42).into()])
        );
    }

    #[test]
    fn parses_array_expression_with_multiple_elements() {
        let tokens = test_tokens![
            Token::LeftSquareBracket,
            Token::Number(1),
            Token::Comma,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(create_array(expr()), &tokens),
            Expression::CreateArray(vec![
                Expression::Number(1).into(),
                Expression::Ident(test_ident("x")).into(),
                Expression::BinaryOp {
                    lhs: Expression::Number(2).into(),
                    kind: BinaryOpKind::Add,
                    rhs: Expression::Number(3).into(),
                }
                .into(),
            ])
        );
    }
}
