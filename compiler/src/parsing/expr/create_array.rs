use chumsky::{IterParser, Parser, primitive::just};
use tap::TapFallible;

use crate::{ast::expression::Expression, lexing::token::Token};

pub(super) fn create_array<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let elements = expr.separated_by(just(Token::Comma)).collect::<Vec<_>>();

    just(Token::LeftSquareBracket)
        .ignore_then(elements)
        .then_ignore(just(Token::RightSquareBracket))
        .map(|elements| Expression::CreateArray(elements.into_iter().map(Box::new).collect()))
}
