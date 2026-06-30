use chumsky::prelude::select;

use crate::{ast::ty::Type, lexing::token::Token};

pub(super) fn atom<'a>() -> parser_type!(Type) {
    select! {
        Token::I32 => Type::I32,
        Token::U32 => Type::U32,
        Token::String => Type::String,
        Token::Ident(ident) => Type::Named(ident),
        Token::Bool => Type::Bool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{test_parse, test_tokens};

    #[test]
    fn parses_i32_ty() {
        let tokens = test_tokens![Token::I32];

        assert_eq!(test_parse(atom(), &tokens), Type::I32);
    }

    #[test]
    fn parses_string_ty() {
        let tokens = test_tokens![Token::String];

        assert_eq!(test_parse(atom(), &tokens), Type::String);
    }

    #[test]
    fn parses_u32_ty() {
        let tokens = test_tokens![Token::U32];

        assert_eq!(test_parse(atom(), &tokens), Type::U32);
    }

    #[test]
    fn parses_bool_ty() {
        let tokens = test_tokens![Token::Bool];

        assert_eq!(test_parse(atom(), &tokens), Type::Bool);
    }
}
