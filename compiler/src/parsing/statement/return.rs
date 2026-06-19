use chumsky::{Parser, prelude::just};

use crate::{ast::statement::Statement, lexing::token::Token, parsing::expr::expr};

pub(super) fn return_stmt<'a>() -> parser_type!(Statement) {
    just(Token::Return)
        .ignore_then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Return)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{expression::Expression, statement::Statement},
        parsing::{test_parse, test_tokens},
    };

    #[test]
    fn parses_return_stmt() {
        let tokens = test_tokens![Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            test_parse(return_stmt(), &tokens),
            Statement::Return(Expression::Number(42))
        );
    }
}
