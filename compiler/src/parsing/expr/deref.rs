use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::expr::expr};

pub(super) fn deref_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    just(Token::Multiply)
        .ignore_then(expr)
        .map(|ptr| Expression::Deref { ptr: Box::new(ptr) })
}
