use chumsky::{
    Parser,
    prelude::{choice, just},
    select,
};

use crate::{
    ast::expression::{Expression, MathOpKind},
    lexing::token::Token,
    parsing::{ident::ident, parsing_rule},
};

parsing_rule! {
    expr -> Expression {
        choice((function_call_expr(), math_op_expr(), base_expr()))
    }
}

parsing_rule! {
    function_call_expr -> Expression {
        ident()
            .then_ignore(just(Token::LeftParen))
            .then_ignore(just(Token::RightParen))
            .map(|name| Expression::FunctionCall {
                name, args: vec![]
            })
    }
}

parsing_rule! {
    base_expr -> Expression {
        select! {
            Token::Number(value) => Expression::Number(value),
            Token::Ident(name) => Expression::Ident(name.clone()),
        }
    }
}

parsing_rule! {
    math_op_expr -> Expression {
        base_expr().foldl(
            math_op_kind()
            .then(base_expr()).repeated(),
            |lhs, (op_kind, rhs)| Expression::MathOp {
                lhs: lhs.into(),
                kind: op_kind,
                rhs: rhs.into()
            })
    }
}

parsing_rule! {
    math_op_kind -> MathOpKind {
        select! {
            Token::Add => MathOpKind::Add,
            Token::Minus => MathOpKind::Subtract,
            Token::Divide => MathOpKind::Divide,
            Token::Multiply => MathOpKind::Multiply
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_expr() {
        use chumsky::Parser as _;

        let tokens = [Token::Number(1), Token::Add, Token::Number(2)];
        let call_tokens = [
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::RightParen,
        ];

        assert_eq!(
            expr().parse(&tokens).unwrap(),
            Expression::MathOp {
                lhs: Expression::Number(1).into(),
                kind: MathOpKind::Add,
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
    fn parses_function_call_expr() {
        use chumsky::Parser as _;

        let tokens = [
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::RightParen,
        ];

        assert_eq!(
            function_call_expr().parse(&tokens).unwrap(),
            Expression::FunctionCall {
                name: "test".into(),
                args: vec![]
            }
        );
    }

    #[test]
    fn parses_base_expr() {
        use chumsky::Parser as _;

        let number_tokens = [Token::Number(42)];
        let ident_tokens = [Token::Ident("x".into())];

        assert_eq!(
            base_expr().parse(&number_tokens).unwrap(),
            Expression::Number(42)
        );
        assert_eq!(
            base_expr().parse(&ident_tokens).unwrap(),
            Expression::Ident("x".into())
        );
    }

    #[test]
    fn parses_math_op_expr() {
        use chumsky::Parser as _;

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
        use chumsky::Parser as _;

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
