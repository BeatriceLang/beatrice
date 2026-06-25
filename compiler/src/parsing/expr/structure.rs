use chumsky::{IterParser, Parser, primitive::just};

use crate::{
    ast::expression::Expression,
    lexing::token::Token,
    parsing::{expr::expr, ident::ident},
};

pub(super) fn structure<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let field = ident()
        .then_ignore(just(Token::Colon))
        .then(expr)
        .then_ignore(just(Token::Comma));
    let body = field
        .repeated()
        .collect::<Vec<_>>()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

    ident().then(body).map(|(name, fields)| Expression::Struct {
        name,
        fields: fields
            .into_iter()
            .map(|(i, expr)| (i, Box::new(expr)))
            .collect(),
    })
}
