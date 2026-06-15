use std::path::PathBuf;

use chumsky::{Parser, primitive::just};

use crate::{
    ast::{Item, expression::Expression},
    lexing::token::Token,
    parsing::expr::expr,
};

pub(super) fn import<'a>() -> parser_type!(Item) {
    just(Token::Import).ignore_then(expr()).map(|expr| {
        let Expression::StringLiteral(path) = expr else {
            todo!("Handle error")
        };

        Item::Import(path.into())
    })
}
