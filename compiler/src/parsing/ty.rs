use chumsky::{
    Parser,
    prelude::select,
    primitive::{choice, just},
    recursive::recursive,
};

use crate::{
    ast::{expression::Expression, ty::Type},
    lexing::token::Token,
    parsing::expr::expr,
};

pub fn ty<'a>() -> parser_type!(Type) {
    recursive(|ty| {
        let atom = select! {
            Token::I32 => Type::I32,
            Token::U32 => Type::U32,
            Token::String => Type::String,
            Token::Ident(ident) => Type::Struct(ident),
            Token::Bool => Type::Bool,
        };

        let array = just(Token::LeftSquareBracket)
            .ignore_then(ty.clone())
            .then_ignore(just(Token::Semicolon))
            .then(select! { Token::Number(size) => size })
            .then_ignore(just(Token::RightSquareBracket))
            .map(|(element_ty, size)| Type::Array {
                element_ty: Box::new(element_ty),
                size: size.try_into().unwrap(),
            });

        let ptr = just(Token::Multiply)
            .ignore_then(ty)
            .map(|ty| Type::Ptr(Box::new(ty)));

        choice((array, atom, ptr))
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
    fn parses_bool_ty() {
        use crate::parsing::{test_parse, test_tokens};

        let tokens = test_tokens![Token::Bool];

        assert_eq!(test_parse(ty(), &tokens), Type::Bool);
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
