use chumsky::{Parser, prelude::just};

use crate::{ast::ty::Type, lexing::token::Token};

pub(super) fn ptr<'a>(ty: parser_type!(Type)) -> parser_type!(Type) {
    just(Token::Multiply)
        .ignore_then(ty)
        .map(|ty| Type::Ptr(Box::new(ty)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{test_parse, test_tokens, ty::ty};

    #[test]
    fn parses_ptr_ty() {
        let tokens = test_tokens![Token::Multiply, Token::I32];

        assert_eq!(
            test_parse(ptr(ty()), &tokens),
            Type::Ptr(Box::new(Type::I32))
        );
    }

    #[test]
    fn parses_nested_ptr_ty() {
        let tokens = test_tokens![Token::Multiply, Token::Multiply, Token::I32];

        assert_eq!(
            test_parse(ptr(ty()), &tokens),
            Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32))))
        );
    }
}
