use chumsky::prelude::choice;

use crate::ast::{Block, statement::Statement};

use assign::assign;
use expr::expr_stmt;
use r#if::r#if;
use r#let::r#let;
use r#return::r#return;
use var::var;
use r#while::r#while;

mod assign;
mod expr;
mod r#if;
mod r#let;
mod r#return;
mod var;
mod r#while;

pub fn stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    choice((
        var(),
        r#let(),
        assign(),
        r#return(),
        expr_stmt(),
        r#if(block.clone()),
        r#while(block),
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
            Statement::Return(Some(Expression::Number(42)))
        );
    }
}
