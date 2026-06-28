use chumsky::{Parser, prelude::just};

use crate::{ast::statement::Statement, lexing::token::Token, parsing::expr::expr};

pub(super) fn expr_stmt<'a>() -> parser_type!(Statement) {
    expr()
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Expression)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{expression::Expression, statement::Statement},
        parsing::{test_parse, test_tokens},
    };

    #[test]
    fn parses_expr_stmt() {
        let tokens = test_tokens![Token::Number(42), Token::Semicolon];

        assert_eq!(
            test_parse(expr(), &tokens),
            Statement::Expression(Expression::Number(42))
        );
    }
}
