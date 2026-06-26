use chumsky::{
    prelude::select,
    primitive::{choice, just},
    recursive::recursive,
    Parser,
};

use crate::{ast::Type, lexing::token::Token};

pub fn ty<'a>() -> parser_type!(Type) {
    recursive(|ty| {
        let atom = select! {
            Token::I32 => Type::I32,
            Token::U32 => Type::U32,
            Token::String => Type::String,
            Token::Ident(ident) => Type::Struct(ident)
        };

        let ptr = just(Token::Multiply)
            .ignore_then(ty)
            .map(|ty| Type::Ptr(Box::new(ty)));

        choice((atom, ptr))
    })
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

    #[test]
    fn parses_u32_ty() {
        use crate::parsing::{test_parse, test_tokens};

        let tokens = test_tokens![Token::U32];

        assert_eq!(test_parse(ty(), &tokens), Type::U32);
    }

    #[test]
    fn parses_ptr_ty() {
        use crate::parsing::{test_parse, test_tokens};

        let tokens = test_tokens![Token::Multiply, Token::I32];

        assert_eq!(test_parse(ty(), &tokens), Type::Ptr(Box::new(Type::I32)));
    }

    #[test]
    fn parses_nested_ptr_ty() {
        use crate::parsing::{test_parse, test_tokens};

        let tokens = test_tokens![Token::Multiply, Token::Multiply, Token::I32];

        assert_eq!(
            test_parse(ty(), &tokens),
            Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32))))
        );
    }
}
