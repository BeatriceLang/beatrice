use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ty::ty};

pub(super) fn cast<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    expr.then_ignore(just(Token::As))
        .then(ty())
        .map(|(value, to)| Expression::Cast {
            value: Box::new(value),
            to,
        })
}
