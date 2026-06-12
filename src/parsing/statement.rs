use chumsky::{
    Parser,
    prelude::{choice, just},
};

use crate::{ast::statement::Statement, lexing::token::Token, parsing::expr::expr};

pub fn return_stmt<'a>() -> parser_type!(Statement) {
    just(Token::Return)
        .ignore_then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Return)
}

pub fn expr_stmt<'a>() -> parser_type!(Statement) {
    expr()
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Expression)
}

pub fn stmt<'a>() -> parser_type!(Statement) {
    choice((return_stmt(), expr_stmt()))
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
            stmt().parse(&tokens).unwrap(),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }
}
