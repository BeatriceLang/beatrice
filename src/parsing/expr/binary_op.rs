use chumsky::{Parser, select};

use crate::{
    ast::expression::{BinaryOpKind, Expression},
    lexing::token::Token,
    parsing::expr::primary::primary_expr,
};

pub fn binary_op_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    primary_expr(expr.clone()).foldl(
        binary_op_kind().then(expr).repeated(),
        |lhs, (op_kind, rhs)| Expression::BinaryOp {
            lhs: lhs.into(),
            kind: op_kind,
            rhs: rhs.into(),
        },
    )
}

pub fn binary_op_kind<'a>() -> parser_type!(BinaryOpKind) {
    select! {
        Token::Add => BinaryOpKind::Add,
        Token::Minus => BinaryOpKind::Subtract,
        Token::Divide => BinaryOpKind::Divide,
        Token::Multiply => BinaryOpKind::Multiply,
        Token::LessThan => BinaryOpKind::LessThan,
        Token::GreaterThan => BinaryOpKind::GreaterThan,
        Token::Equal => BinaryOpKind::EqualTo,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        lexing::token::Token,
        parsing::{
            expr::{
                binary_op::{binary_op_expr, binary_op_kind},
                expr,
            },
            test_parse, test_tokens,
        },
    };

    #[test]
    fn parses_binary_op_expr() {
        let tokens = test_tokens![Token::Number(8), Token::Divide, Token::Number(2)];
        let condition_tokens =
            test_tokens![Token::Ident("n".into()), Token::LessThan, Token::Number(2)];
        let chained_tokens = test_tokens![
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
        ];

        assert_eq!(
            test_parse(binary_op_expr(expr()), &tokens),
            Expression::BinaryOp {
                lhs: Expression::Number(8).into(),
                kind: BinaryOpKind::Divide,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            test_parse(binary_op_expr(expr()), &condition_tokens),
            Expression::BinaryOp {
                lhs: Expression::Ident(test_ident("n")).into(),
                kind: BinaryOpKind::LessThan,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            test_parse(binary_op_expr(expr()), &chained_tokens),
            Expression::BinaryOp {
                lhs: Expression::Number(1).into(),
                kind: BinaryOpKind::Add,
                rhs: Expression::BinaryOp {
                    lhs: Expression::Number(2).into(),
                    kind: BinaryOpKind::Add,
                    rhs: Expression::Number(3).into(),
                }
                .into(),
            }
        );
    }

    #[test]
    fn parses_binary_op_kind() {
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::Add]),
            BinaryOpKind::Add
        );
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::Minus]),
            BinaryOpKind::Subtract
        );
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::Divide]),
            BinaryOpKind::Divide
        );
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::Multiply]),
            BinaryOpKind::Multiply
        );
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::LessThan]),
            BinaryOpKind::LessThan
        );
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::GreaterThan]),
            BinaryOpKind::GreaterThan
        );
        assert_eq!(
            test_parse(binary_op_kind(), &test_tokens![Token::Equal]),
            BinaryOpKind::EqualTo
        );
    }
}
