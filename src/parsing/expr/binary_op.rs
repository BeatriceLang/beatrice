use chumsky::{Parser, select};

use crate::{
    ast::expression::{BinaryOpKind, Expression},
    lexing::token::Token,
    parsing::expr::base::base_expr,
};

pub fn binary_op_expr<'a>() -> parser_type!(Expression) {
    base_expr().foldl(
        binary_op_kind().then(base_expr()).repeated(),
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
        Token::Multiply => BinaryOpKind::Multiply
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        lexing::token::Token,
        parsing::expr::binary_op::{binary_op_expr, binary_op_kind},
    };

    #[test]
    fn parses_binary_op_expr() {
        let tokens = [Token::Number(8), Token::Divide, Token::Number(2)];
        let chained_tokens = [
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
        ];

        assert_eq!(
            binary_op_expr().parse(&tokens).unwrap(),
            Expression::BinaryOp {
                lhs: Expression::Number(8).into(),
                kind: BinaryOpKind::Divide,
                rhs: Expression::Number(2).into(),
            }
        );

        assert_eq!(
            binary_op_expr().parse(&chained_tokens).unwrap(),
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
    fn parses_binary_op_kind() {
        assert_eq!(
            binary_op_kind().parse(&[Token::Add]).unwrap(),
            BinaryOpKind::Add
        );
        assert_eq!(
            binary_op_kind().parse(&[Token::Minus]).unwrap(),
            BinaryOpKind::Subtract
        );
        assert_eq!(
            binary_op_kind().parse(&[Token::Divide]).unwrap(),
            BinaryOpKind::Divide
        );
        assert_eq!(
            binary_op_kind().parse(&[Token::Multiply]).unwrap(),
            BinaryOpKind::Multiply
        );
    }
}
