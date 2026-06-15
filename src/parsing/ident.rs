use chumsky::{Parser, input::MapExtra, prelude::select, span::SimpleSpan};

use crate::{ast::Ident, lexing::token::Token};

pub fn ident<'a>() -> parser_type!(Ident) {
    select! {
        Token::Ident(name) => name.clone()
    }
    .map_with(
        |name,
         e: &mut MapExtra<
            'a,
            '_,
            chumsky::input::MappedInput<'a, Token, SimpleSpan, &'a [(Token, SimpleSpan)]>,
            chumsky::extra::Err<chumsky::error::Rich<'a, Token, SimpleSpan>>,
        >| { Ident::new(name, e.span().into_range()) },
    )
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
