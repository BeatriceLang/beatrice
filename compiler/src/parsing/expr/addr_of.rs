use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn addr_of_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    just(Token::AddressOf)
        .ignore_then(expr)
        .map(|value| Expression::AddressOf {
            value: Box::new(value),
        })
}
