use chumsky::{
    primitive::{choice, just},
    Parser,
};

use crate::{
    ast::expression::{BinaryOpKind, Expression},
    lexing::token::Token,
    parsing::expr::primary::primary_expr,
};

pub fn binary_op_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let primary = primary_expr(expr);

    let product = primary.clone().foldl(
        choice((
            just(Token::Multiply).to(BinaryOpKind::Multiply),
            just(Token::Divide).to(BinaryOpKind::Divide),
        ))
        .then(primary)
        .repeated(),
        binary_op,
    );

    let sum = product.clone().foldl(
        choice((
            just(Token::Add).to(BinaryOpKind::Add),
            just(Token::Minus).to(BinaryOpKind::Subtract),
        ))
        .then(product)
        .repeated(),
        binary_op,
    );

    let comparison = sum.clone().foldl(
        choice((
            just(Token::LessThan).to(BinaryOpKind::LessThan),
            just(Token::GreaterThan).to(BinaryOpKind::GreaterThan),
        ))
        .then(sum)
        .repeated(),
        binary_op,
    );

    comparison.clone().foldl(
        just(Token::Equal)
            .to(BinaryOpKind::EqualTo)
            .then(comparison)
            .repeated(),
        binary_op,
    )
}

fn binary_op(lhs: Expression, (kind, rhs): (BinaryOpKind, Expression)) -> Expression {
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
            expr::{binary_op::binary_op_expr, expr},
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
            test_parse(binary_op_expr(expr()), &product_before_sum),
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
            test_parse(binary_op_expr(expr()), &sum_before_comparison),
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
            test_parse(binary_op_expr(expr()), &comparison_before_equality),
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
