use chumsky::{Parser, select};

use crate::{
    ast::expression::{Expression, MathOpKind},
    lexing::token::Token,
    parsing::expr::base::base_expr,
};

pub fn math_op_expr<'a>() -> parser_type!(Expression) {
    base_expr().foldl(
        math_op_kind().then(base_expr()).repeated(),
        |lhs, (op_kind, rhs)| Expression::MathOp {
            lhs: lhs.into(),
            kind: op_kind,
            rhs: rhs.into(),
        },
    )
}

pub fn math_op_kind<'a>() -> parser_type!(MathOpKind) {
    select! {
        Token::Add => MathOpKind::Add,
        Token::Minus => MathOpKind::Subtract,
        Token::Divide => MathOpKind::Divide,
        Token::Multiply => MathOpKind::Multiply
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use crate::{
        ast::expression::{Expression, MathOpKind},
        lexing::token::Token,
        parsing::expr::math_op::{math_op_expr, math_op_kind},
    };

    #[test]
    fn parses_math_op_expr() {
        let tokens = [Token::Number(8), Token::Divide, Token::Number(2)];
        let chained_tokens = [
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
        ];

        assert_eq!(
            math_op_expr().parse(&tokens).unwrap(),
            Expression::MathOp {
                lhs: Expression::Number(8).into(),
                kind: MathOpKind::Divide,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            math_op_expr().parse(&chained_tokens).unwrap(),
            Expression::MathOp {
                lhs: Expression::MathOp {
                    lhs: Expression::Number(1).into(),
                    kind: MathOpKind::Add,
                    rhs: Expression::Number(2).into(),
                }
                .into(),
                kind: MathOpKind::Add,
                rhs: Expression::Number(3).into(),
            }
        );
    }

    #[test]
    fn parses_math_op_kind() {
        assert_eq!(
            math_op_kind().parse(&[Token::Add]).unwrap(),
            MathOpKind::Add
        );
        assert_eq!(
            math_op_kind().parse(&[Token::Minus]).unwrap(),
            MathOpKind::Subtract
        );
        assert_eq!(
            math_op_kind().parse(&[Token::Divide]).unwrap(),
            MathOpKind::Divide
        );
        assert_eq!(
            math_op_kind().parse(&[Token::Multiply]).unwrap(),
            MathOpKind::Multiply
        );
    }
}
