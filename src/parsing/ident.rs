use chumsky::prelude::select;

use crate::lexing::token::Token;

pub fn ident<'a>() -> parser_type!(String) {
    select! {
        Token::Ident(name) => name.clone()
    }
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
