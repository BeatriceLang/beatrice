use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn invert<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    just(Token::ExclamationMark)
        .ignore_then(expr)
        .map(|val| Expression::Invert(Box::new(val)))
}
