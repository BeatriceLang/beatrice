use chumsky::prelude::choice;

use crate::ast::{statement::Statement, Block};

mod assign;
mod expr;
mod r#if;
mod r#let;
mod r#return;
mod var;
mod r#while;

pub fn stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    choice((
        var::var_stmt(),
        r#let::let_stmt(),
        assign::assign_stmt(),
        r#return::return_stmt(),
        expr::expr_stmt(),
        r#if::if_stmt(block.clone()),
        r#while::while_stmt(block),
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
