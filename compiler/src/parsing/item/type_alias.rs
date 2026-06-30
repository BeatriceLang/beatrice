use chumsky::{Parser, primitive::just};

use crate::{
    ast::item::Item,
    lexing::token::Token,
    parsing::{ident::ident, ty::ty},
};

pub(super) fn type_alias<'a>() -> parser_type!(Item) {
    just(Token::Type)
        .ignore_then(ident())
        .then_ignore(just(Token::Assign))
        .then(ty())
        .map(|(alias, ty)| Item::TypeAlias { alias, ty })
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use super::*;
    use crate::{
        ast::ty::Type,
        parsing::{test_ident, test_input, test_parse, test_tokens},
    };

    #[test]
    fn parses_type_alias_item() {
        let tokens = test_tokens![
            Token::Type,
            Token::Ident("Count".into()),
            Token::Assign,
            Token::I32,
        ];

        assert_eq!(
            test_parse(type_alias(), &tokens),
            Item::TypeAlias {
                alias: test_ident("Count"),
                ty: Type::I32,
            }
        );
    }

    #[test]
    fn rejects_type_alias_without_assign() {
        let tokens = test_tokens![
            Token::Type,
            Token::Ident("Count".into()),
            Token::I32,
        ];

        let errors = type_alias().parse(test_input(&tokens)).into_errors();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].span().into_range(), 2..3);
    }
}
