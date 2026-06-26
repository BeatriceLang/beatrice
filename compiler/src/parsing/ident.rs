use chumsky::{
    Parser,
    error::Rich,
    input::{MapExtra, MappedInput},
    prelude::select,
    span::SimpleSpan,
};

use crate::{ast::Ident, lexing::token::Token};

type IdentInput<'a> = MappedInput<'a, Token, SimpleSpan, &'a [(Token, SimpleSpan)]>;
type IdentMapExtra<'a, 'parse> =
    MapExtra<'a, 'parse, IdentInput<'a>, chumsky::extra::Err<Rich<'a, Token, SimpleSpan>>>;

pub fn ident<'a>() -> parser_type!(Ident) {
    select! {
        Token::Ident(name) => name
    }
    .map_with(|name, e: &mut IdentMapExtra<'a, '_>| Ident::new(name, e.span().into_range()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ident() {
        use crate::parsing::test_ident;
        use chumsky::Parser as _;

        let tokens = crate::parsing::test_tokens![Token::Ident("main".into())];

        let ident = ident().parse(crate::parsing::test_input(&tokens)).unwrap();

        assert_eq!(ident, test_ident("main"));
        assert_eq!(ident.span(), 0..1);
    }
}
