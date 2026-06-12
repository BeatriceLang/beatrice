use chumsky::prelude::{Parser, select};

use crate::{ast::Type, lexing::token::Token, parsing::BeatriceParser};

pub fn ty<'a>() -> BeatriceParser<'a, Type> {
    select! {
        Token::I32 => Type::I32
    }
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ty() {
        use chumsky::Parser as _;

        let tokens = [Token::I32];

        assert_eq!(ty().parse(&tokens).unwrap(), Type::I32);
    }
}
