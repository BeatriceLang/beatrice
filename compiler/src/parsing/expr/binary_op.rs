use chumsky::{
    Parser,
    pratt::{infix, left},
    primitive::{choice, just},
};

use crate::{
    ast::expression::{BinaryOpKind, Expression},
    lexing::token::Token,
    parsing::expr::atom::atom,
};

pub fn binary_op<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let primary = atom(expr);

    primary.pratt((
        infix(
            left(4),
            just(Token::Multiply).to(BinaryOpKind::Multiply),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
        infix(
            left(4),
            just(Token::Divide).to(BinaryOpKind::Divide),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
        infix(
            left(3),
            just(Token::Add).to(BinaryOpKind::Add),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
        infix(
            left(3),
            just(Token::Minus).to(BinaryOpKind::Subtract),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
        infix(
            left(2),
            just(Token::LessThan).to(BinaryOpKind::LessThan),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
        infix(
            left(2),
            just(Token::GreaterThan).to(BinaryOpKind::GreaterThan),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
        infix(
            left(1),
            just(Token::Equal).to(BinaryOpKind::EqualTo),
            |lhs, kind, rhs, _| build_binary_op(lhs, kind, rhs),
        ),
    ))
}

fn build_binary_op(lhs: Expression, kind: BinaryOpKind, rhs: Expression) -> Expression {
    Expression::BinaryOp {
        lhs: lhs.into(),
        kind,
        rhs: rhs.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        lexing::token::Token,
        parsing::{
            expr::{binary_op::binary_op, expr},
            test_ident, test_parse, test_tokens,
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
            test_parse(binary_op(expr()), &tokens),
            Expression::BinaryOp {
                lhs: Expression::Number(8).into(),
                kind: BinaryOpKind::Divide,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            test_parse(binary_op(expr()), &condition_tokens),
            Expression::BinaryOp {
                lhs: Expression::Ident(test_ident("n")).into(),
                kind: BinaryOpKind::LessThan,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            test_parse(binary_op(expr()), &chained_tokens),
            Expression::BinaryOp {
                lhs: Expression::BinaryOp {
                    lhs: Expression::Number(1).into(),
                    kind: BinaryOpKind::Add,
                    rhs: Expression::Number(2).into(),
                }
                .into(),
                kind: BinaryOpKind::Add,
                rhs: Expression::Number(3).into(),
            }
        );
    }

    #[test]
    fn parses_binary_op_precedence() {
        let product_before_sum = test_tokens![
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::Multiply,
            Token::Number(3),
        ];
        let sum_before_comparison = test_tokens![
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::GreaterThan,
            Token::Number(3),
        ];
        let comparison_before_equality = test_tokens![
            Token::Number(1),
            Token::LessThan,
            Token::Number(2),
            Token::Equal,
            Token::Number(3),
        ];

        assert_eq!(
            test_parse(binary_op(expr()), &product_before_sum),
            Expression::BinaryOp {
                lhs: Expression::Number(1).into(),
                kind: BinaryOpKind::Add,
                rhs: Expression::BinaryOp {
                    lhs: Expression::Number(2).into(),
                    kind: BinaryOpKind::Multiply,
                    rhs: Expression::Number(3).into(),
                }
                .into(),
            }
        );

        assert_eq!(
            test_parse(binary_op(expr()), &sum_before_comparison),
            Expression::BinaryOp {
                lhs: Expression::BinaryOp {
                    lhs: Expression::Number(1).into(),
                    kind: BinaryOpKind::Add,
                    rhs: Expression::Number(2).into(),
                }
                .into(),
                kind: BinaryOpKind::GreaterThan,
                rhs: Expression::Number(3).into(),
            }
        );

        assert_eq!(
            test_parse(binary_op(expr()), &comparison_before_equality),
            Expression::BinaryOp {
                lhs: Expression::BinaryOp {
                    lhs: Expression::Number(1).into(),
                    kind: BinaryOpKind::LessThan,
                    rhs: Expression::Number(2).into(),
                }
                .into(),
                kind: BinaryOpKind::EqualTo,
                rhs: Expression::Number(3).into(),
            }
        );
    }
}
