use chumsky::prelude::recursive;

use crate::{
    ast::expression::Expression,
    parsing::expr::{binary_op::binary_op, postfix::postfix_expr},
};

mod atom;
mod binary_op;
mod postfix;

pub fn expr<'a>() -> parser_type!(Expression) {
    recursive(binary_op)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        lexing::token::Token,
        parsing::{expr::expr, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_expr() {
        let tokens = test_tokens![Token::Number(1), Token::Add, Token::Number(2)];
        let call_tokens = test_tokens![
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::RightParen,
        ];

        assert_eq!(
            test_parse(expr(), &tokens),
            Expression::BinaryOp {
                lhs: Expression::Number(1).into(),
                kind: BinaryOpKind::Add,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            test_parse(expr(), &call_tokens),
            Expression::FunctionCall {
                name: test_ident("test"),
                args: vec![]
            }
        );
    }

    #[test]
    fn parses_expr_function_call_with_args() {
        let tokens = test_tokens![
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
            Token::RightParen,
        ];

        assert_eq!(
            test_parse(expr(), &tokens),
            Expression::FunctionCall {
                name: test_ident("test"),
                args: vec![
                    Expression::Number(1),
                    Expression::BinaryOp {
                        lhs: Expression::Number(2).into(),
                        kind: BinaryOpKind::Add,
                        rhs: Expression::Number(3).into(),
                    },
                ],
            }
        );
    }

    #[test]
    fn parses_cast_with_binary_op_rhs() {
        let tokens = test_tokens![
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::As,
            Token::I32,
        ];

        assert_eq!(
            test_parse(expr(), &tokens),
            Expression::BinaryOp {
                lhs: Expression::Number(1).into(),
                kind: BinaryOpKind::Add,
                rhs: Expression::Cast {
                    value: Expression::Number(2).into(),
                    to: crate::ast::ty::Type::I32,
                }
                .into(),
            }
        );
    }

    #[test]
    #[ignore = "array access is not yet part of binary operator operands"]
    fn parses_binary_op_with_array_access_operands() {
        let tokens = test_tokens![
            Token::Ident("values".into()),
            Token::LeftSquareBracket,
            Token::Number(0),
            Token::RightSquareBracket,
            Token::Equal,
            Token::Ident("values".into()),
            Token::LeftSquareBracket,
            Token::Number(1),
            Token::RightSquareBracket,
        ];

        assert_eq!(
            test_parse(expr(), &tokens),
            Expression::BinaryOp {
                lhs: Expression::ArrayAccess {
                    array: Expression::Ident(test_ident("values")).into(),
                    index: Expression::Number(0).into(),
                }
                .into(),
                kind: BinaryOpKind::EqualTo,
                rhs: Expression::ArrayAccess {
                    array: Expression::Ident(test_ident("values")).into(),
                    index: Expression::Number(1).into(),
                }
                .into(),
            }
        );
    }
}
