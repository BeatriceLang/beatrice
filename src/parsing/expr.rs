use chumsky::{
    Parser,
    prelude::choice,
    select,
};

use crate::{
    ast::expression::{Expression, MathOpKind},
    lexing::token::Token,
    parsing::parsing_rule,
};

parsing_rule! {
    expr -> Expression {
        choice((math_op_expr(), base_expr()))
    }

    test {
        use chumsky::Parser as _;

        let tokens = [Token::Number(1), Token::Add, Token::Number(2)];

        assert_eq!(
            expr().parse(&tokens).unwrap(),
            Expression::MathOp {
                lhs: Expression::Number(1).into(),
                kind: MathOpKind::Add,
                rhs: Expression::Number(2).into(),
            }
        );
    }
}

parsing_rule! {
    base_expr -> Expression {
        select! {
            Token::Number(value) => Expression::Number(value),
            Token::Ident(name) => Expression::Ident(name.clone()),
        }
    }

    test {
        use chumsky::Parser as _;

        let number_tokens = [Token::Number(42)];
        let ident_tokens = [Token::Ident("x".into())];

        assert_eq!(base_expr().parse(&number_tokens).unwrap(), Expression::Number(42));
        assert_eq!(
            base_expr().parse(&ident_tokens).unwrap(),
            Expression::Ident("x".into())
        );
    }
}

parsing_rule! {
    math_op_expr -> Expression {
        base_expr()
            .then(math_op_kind())
            .then(base_expr())
            .map(|((lhs, op_kind), rhs)| Expression::MathOp {
                lhs: lhs.into(),
                kind: op_kind,
                rhs: rhs.into()
            })
    }

    test {
        use chumsky::Parser as _;

        let tokens = [Token::Number(8), Token::Divide, Token::Number(2)];

        assert_eq!(
            math_op_expr().parse(&tokens).unwrap(),
            Expression::MathOp {
                lhs: Expression::Number(8).into(),
                kind: MathOpKind::Divide,
                rhs: Expression::Number(2).into(),
            }
        );
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

    test {
        use chumsky::Parser as _;

        assert_eq!(math_op_kind().parse(&[Token::Add]).unwrap(), MathOpKind::Add);
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
