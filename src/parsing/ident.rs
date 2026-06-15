use chumsky::prelude::select;

use crate::{ast::Ident, lexing::token::Token};

pub fn ident<'a>() -> parser_type!(Ident) {
    select! {
        Token::Ident(name) => name.clone()
    }
    .map_with(|name, e| Ident::new(name, e.span().into_range()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ident() {
        use chumsky::Parser as _;

        let tokens = crate::parsing::test_tokens![Token::Ident("main".into())];

        let ident = ident().parse(crate::parsing::test_input(&tokens)).unwrap();

        assert_eq!(ident, test_ident("main"));
        assert_eq!(ident.span(), 0..1);
    }
}
