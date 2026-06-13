use chumsky::{
    Parser,
    prelude::{choice, just},
};

use crate::{
    ast::{Block, statement::Statement},
    lexing::token::Token,
    parsing::{block::block, expr::expr},
};

pub fn return_stmt<'a>() -> parser_type!(Statement) {
    just(Token::Return)
        .ignore_then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Return)
}

pub fn if_stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    just(Token::If)
        .ignore_then(expr())
        .then(block)
        .map(|(cond, body)| Statement::If { cond, body })
}

pub fn expr_stmt<'a>() -> parser_type!(Statement) {
    expr()
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Expression)
}

pub fn stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    choice((return_stmt(), expr_stmt(), if_stmt(block)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_return_stmt() {
        use chumsky::Parser as _;

        let tokens = [Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            return_stmt().parse(&tokens).unwrap(),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }

    #[test]
    fn parses_expr_stmt() {
        use chumsky::Parser as _;

        let tokens = [Token::Number(42), Token::Semicolon];

        assert_eq!(
            expr_stmt().parse(&tokens).unwrap(),
            Statement::Expression(crate::ast::expression::Expression::Number(42))
        );
    }

    #[test]
    fn parses_stmt() {
        use chumsky::Parser as _;

        let tokens = [Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            stmt(block()).parse(&tokens).unwrap(),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }

    #[test]
    fn parses_if_stmt() {
        use chumsky::Parser as _;

        let tokens = [
            Token::If,
            Token::Ident("n".into()),
            Token::LessThan,
            Token::Number(2),
            Token::LeftBrace,
            Token::Return,
            Token::Ident("n".into()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(
            if_stmt(block()).parse(&tokens).unwrap(),
            Statement::If {
                cond: crate::ast::expression::Expression::BinaryOp {
                    lhs: crate::ast::expression::Expression::Ident("n".into()).into(),
                    kind: crate::ast::expression::BinaryOpKind::LessThan,
                    rhs: crate::ast::expression::Expression::Number(2).into(),
                },
                body: crate::ast::Block {
                    statements: vec![Statement::Return(
                        crate::ast::expression::Expression::Ident("n".into())
                    )],
                },
            }
        );
    }
}
