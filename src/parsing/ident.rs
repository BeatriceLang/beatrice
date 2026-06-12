use chumsky::prelude::{Parser, select};

use crate::{lexing::token::Token, parsing::BeatriceParser};

pub fn ident<'a>() -> BeatriceParser<'a, String> {
    select! {
        Token::Ident(name) => name.clone()
    }
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ident() {
        use chumsky::Parser as _;

        let tokens = [Token::Ident("main".into())];

        assert_eq!(ident().parse(&tokens).unwrap(), "main");
    }
}
