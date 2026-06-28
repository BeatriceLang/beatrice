use chumsky::{Parser, prelude::just};

use crate::{ast::statement::Statement, lexing::token::Token, parsing::expr::expr};

pub(super) fn r#return<'a>() -> parser_type!(Statement) {
    just(Token::Return)
        .ignore_then(expr().or_not())
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
            test_parse(r#return(), &tokens),
            Statement::Return(Some(Expression::Number(42)))
        );
    }

    #[test]
    fn parses_return_stmt_without_value() {
        let tokens = test_tokens![Token::Return, Token::Semicolon];

        assert_eq!(test_parse(r#return(), &tokens), Statement::Return(None));
    }
}
