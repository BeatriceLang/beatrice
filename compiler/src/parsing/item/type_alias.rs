use chumsky::{Parser, primitive::just};

use crate::{
    ast::item::Item,
    lexing::token::Token,
    parsing::{ident::ident, ty::ty},
};

pub(super) fn type_alias<'a>() -> parser_type!(Item) {
    just(Token::Type)
        .ignore_then(ident())
        .then_ignore(just(Token::Assign))
        .then(ty())
        .map(|(alias, ty)| Item::TypeAlias { alias, ty })
}
