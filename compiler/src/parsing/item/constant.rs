use chumsky::{Parser, primitive::just};

use crate::{
    ast::Item,
    lexing::token::Token,
    parsing::{expr::expr, ident::ident, ty::ty},
};

pub(super) fn constant<'a>() -> parser_type!(Item) {
    just(Token::Const)
        .ignore_then(ident())
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Assign))
        .then(expr())
        .map(|((name, ty), val)| Item::Const { name, ty, val })
}
