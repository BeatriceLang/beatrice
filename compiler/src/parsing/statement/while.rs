use chumsky::{Parser, primitive::just};

use crate::{
    ast::statement::Statement,
    lexing::token::Token,
    parsing::{block::block, expr::expr},
};

pub(super) fn while_stmt<'a>() -> parser_type!(Statement) {
    just(Token::While)
        .ignore_then(expr())
        .then(block())
        .map(|(cond, body)| Statement::While { cond, body })
}
