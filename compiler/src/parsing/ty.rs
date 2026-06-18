use chumsky::prelude::select;

use crate::{ast::Type, lexing::token::Token};

pub fn ty<'a>() -> parser_type!(Type) {
    select! {
        Token::I32 => Type::I32,
        Token::String => Type::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ty() {
        use crate::parsing::{test_parse, test_tokens};

        let tokens = test_tokens![Token::I32];

        assert_eq!(test_parse(ty(), &tokens), Type::I32);
    }

    #[test]
    fn parses_string_ty() {
        use crate::parsing::{test_parse, test_tokens};

        let tokens = test_tokens![Token::String];

        assert_eq!(test_parse(ty(), &tokens), Type::String);
    }
}
