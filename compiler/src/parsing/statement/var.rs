use chumsky::{Parser, primitive::just};

use crate::{
    ast::statement::Statement,
    lexing::token::Token,
    parsing::{expr::expr, ident::ident, ty::ty},
};

pub(super) fn var<'a>() -> parser_type!(Statement) {
    just(Token::Var)
        .ignore_then(ident())
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Assign))
        .then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(|((name, ty), value)| Statement::Var { name, ty, value })
}
