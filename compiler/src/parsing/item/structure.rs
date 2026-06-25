use chumsky::{IterParser, Parser, primitive::just};

use crate::{
    ast::Struct,
    lexing::token::Token,
    parsing::{ident::ident, ty::ty},
};

pub(super) fn structure<'a>() -> parser_type!(Struct) {
    let field = ident()
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Comma));
    let body = field
        .repeated()
        .collect()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

    just(Token::Struct)
        .ignore_then(ident())
        .then(body)
        .map(|(name, fields)| Struct { name, fields })
}
