use chumsky::{
    Parser,
    prelude::{choice, just},
};

use crate::{
    ast::{Block, statement::Statement},
    lexing::token::Token,
    parsing::{expr::expr, ident::ident, ty::ty},
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

pub fn let_stmt<'a>() -> parser_type!(Statement) {
    just(Token::Let)
        .ignore_then(ident())
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Assign))
        .then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(|((name, ty), value)| Statement::Let { name, ty, value })
}

pub fn expr_stmt<'a>() -> parser_type!(Statement) {
    expr()
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Expression)
}

pub fn stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    choice((let_stmt(), return_stmt(), expr_stmt(), if_stmt(block)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{block::block, test_parse, test_tokens};

    #[test]
    fn parses_return_stmt() {
        let tokens = test_tokens![Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            test_parse(return_stmt(), &tokens),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }

    #[test]
    fn parses_expr_stmt() {
        let tokens = test_tokens![Token::Number(42), Token::Semicolon];

        assert_eq!(
            test_parse(expr_stmt(), &tokens),
            Statement::Expression(crate::ast::expression::Expression::Number(42))
        );
    }

    #[test]
    fn parses_let_stmt() {
        let tokens = test_tokens![
            Token::Let,
            Token::Ident("x".into()),
            Token::Colon,
            Token::I32,
            Token::Assign,
            Token::Number(42),
            Token::Semicolon,
        ];

        assert_eq!(
            test_parse(stmt(block()), &tokens),
            Statement::Let {
                name: test_ident("x"),
                ty: crate::ast::Type::I32,
                value: crate::ast::expression::Expression::Number(42),
            }
        );
    }

    #[test]
    fn parses_stmt() {
        let tokens = test_tokens![Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            test_parse(stmt(block()), &tokens),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }

    #[test]
    fn parses_if_stmt() {
        let tokens = test_tokens![
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
            test_parse(if_stmt(block()), &tokens),
            Statement::If {
                cond: crate::ast::expression::Expression::BinaryOp {
                    lhs: crate::ast::expression::Expression::Ident(test_ident("n")).into(),
                    kind: crate::ast::expression::BinaryOpKind::LessThan,
                    rhs: crate::ast::expression::Expression::Number(2).into(),
                },
                body: crate::ast::Block {
                    statements: vec![Statement::Return(
                        crate::ast::expression::Expression::Ident(test_ident("n"))
                    )],
                },
            }
        );
    }
}
