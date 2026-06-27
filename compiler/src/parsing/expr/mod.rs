use chumsky::prelude::{choice, recursive};

use crate::{
    ast::expression::Expression,
    parsing::expr::{binary_op::binary_op_expr, deref::deref_expr, primary::primary_expr},
};

mod addr_of;
mod binary_op;
mod cast;
mod create_struct;
mod deref;
mod field_access;
mod function_call;
mod invert;
mod primary;

pub fn expr<'a>() -> parser_type!(Expression) {
    recursive(|expr| choice((binary_op_expr(expr.clone()), primary_expr(expr.clone()))))
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
                    to: crate::ast::Type::I32,
                }
                .into(),
            }
        );
    }
}
