use chumsky::prelude::{choice, recursive};

use crate::{
    ast::expression::Expression,
    parsing::expr::{
        base::base_expr, binary_op::binary_op_expr, function_call::function_call_expr,
    },
};

mod base;
mod binary_op;
mod function_call;

pub fn expr<'a>() -> parser_type!(Expression) {
    recursive(|expr| choice((function_call_expr(expr), binary_op_expr(), base_expr())))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        lexing::token::Token,
        parsing::expr::expr,
    };

    #[test]
    fn parses_expr() {
        let tokens = [Token::Number(1), Token::Add, Token::Number(2)];
        let call_tokens = [
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::RightParen,
        ];

        assert_eq!(
            expr().parse(&tokens).unwrap(),
            Expression::BinaryOp {
                lhs: Expression::Number(1).into(),
                kind: BinaryOpKind::Add,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            expr().parse(&call_tokens).unwrap(),
            Expression::FunctionCall {
                name: "test".into(),
                args: vec![]
            }
        );
    }

    #[test]
    fn parses_expr_function_call_with_args() {
        let tokens = [
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
            expr().parse(&tokens).unwrap(),
            Expression::FunctionCall {
                name: "test".into(),
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
}
