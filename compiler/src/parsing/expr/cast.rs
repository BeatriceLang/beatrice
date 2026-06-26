use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ty::ty};

pub(super) fn cast<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    expr.foldl(just(Token::As).ignore_then(ty()).repeated(), |value, to| {
        Expression::Cast {
            value: Box::new(value),
            to,
        }
    })
}
