use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ident::ident};

pub(super) fn field_access<'a>() -> parser_type!(Expression) {
    ident()
        .then_ignore(just(Token::Dot))
        .then(ident())
        .map(|(base, field)| Expression::FieldAccess { base, field })
}
