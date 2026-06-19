use chumsky::prelude::choice;

use crate::ast::{Block, statement::Statement};

mod assign;
mod expr;
mod r#if;
mod r#let;
mod r#return;
mod var;

pub fn stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    choice((
        var_stmt(),
        r#let::let_stmt(),
        assign::assign_stmt(),
        r#return::return_stmt(),
        expr::expr_stmt(),
        r#if::if_stmt(block),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{expression::Expression, statement::Statement},
        lexing::token::Token,
        parsing::{block::block, test_parse, test_tokens},
    };

    #[test]
    fn parses_stmt() {
        let tokens = test_tokens![Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            test_parse(stmt(block()), &tokens),
            Statement::Return(Expression::Number(42))
        );
    }
}
