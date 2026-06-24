use chumsky::{primitive::just, Parser};

use crate::{
    ast::Item,
    lexing::token::Token,
    parsing::{expr::expr, ident::ident, ty::ty},
};

pub(super) fn constant<'a>() -> parser_type!(Item) {
    just(Token::Const)
        .ignore_then(ident())
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Assign))
        .then(expr())
        .map(|((name, ty), val)| Item::Const { name, ty, val })
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use super::*;
    use crate::{
        ast::{expression::Expression, Type},
        parsing::{test_ident, test_input, test_parse, test_tokens},
    };

    #[test]
    fn parses_constant_item() {
        let tokens = test_tokens![
            Token::Const,
            Token::Ident("answer".into()),
            Token::Colon,
            Token::I32,
            Token::Assign,
            Token::Number(42),
        ];

        assert_eq!(
            test_parse(constant(), &tokens),
            Item::Const {
                name: test_ident("answer"),
                ty: Type::I32,
                val: Expression::Number(42),
            }
        );
    }

    #[test]
    fn parses_constant_item_with_string_literal() {
        let tokens = test_tokens![
            Token::Const,
            Token::Ident("message".into()),
            Token::Colon,
            Token::String,
            Token::Assign,
            Token::StringLiteral("hello".into()),
        ];

        assert_eq!(
            test_parse(constant(), &tokens),
            Item::Const {
                name: test_ident("message"),
                ty: Type::String,
                val: Expression::StringLiteral("hello".into()),
            }
        );
    }

    #[test]
    fn parses_constant_item_with_ptr_type() {
        let tokens = test_tokens![
            Token::Const,
            Token::Ident("ptr".into()),
            Token::Colon,
            Token::Multiply,
            Token::I32,
            Token::Assign,
            Token::Ident("value".into()),
        ];

        assert_eq!(
            test_parse(constant(), &tokens),
            Item::Const {
                name: test_ident("ptr"),
                ty: Type::Ptr(Box::new(Type::I32)),
                val: Expression::Ident(test_ident("value")),
            }
        );
    }

    #[test]
    fn rejects_constant_item_without_assign() {
        let tokens = test_tokens![
            Token::Const,
            Token::Ident("answer".into()),
            Token::Colon,
            Token::I32,
            Token::Number(42),
        ];
        let errors = constant().parse(test_input(&tokens)).into_errors();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].span().into_range(), 4..5);
    }
}
