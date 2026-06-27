use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ident::ident};

pub(super) fn array_access<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    ident()
        .then_ignore(just(Token::LeftSquareBracket))
        .then(expr)
        .then_ignore(just(Token::RightSquareBracket))
        .map(|(array, index)| Expression::ArrayAccess {
            array,
            index: Box::new(index),
        })
}
