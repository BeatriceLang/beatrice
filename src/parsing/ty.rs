use chumsky::prelude::select;

use crate::{ast::Type, lexing::token::Token};

pub fn ty<'a>() -> parser_type!(Type) {
    select! {
        Token::I32 => Type::I32
    }
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
