use chumsky::{Parser, prelude::just};

use crate::{
    ast::statement::Statement,
    lexing::token::Token,
    parsing::{expr::expr, ident::ident, ty::ty},
};

pub(super) fn let_stmt<'a>() -> parser_type!(Statement) {
    just(Token::Let)
        .ignore_then(ident())
        .then_ignore(just(Token::Colon))
        .then(ty())
        .then_ignore(just(Token::Assign))
        .then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(|((name, ty), value)| Statement::Let { name, ty, value })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{expression::Expression, statement::Statement, ty::Type},
        parsing::{test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_let_stmt() {
        let tokens = test_tokens![
            Token::Let,
            Token::Ident("x".into()),
            Token::Colon,
            Token::I32,
            Token::Assign,
            Token::Number(42),
            Token::Semicolon,
        ];

        assert_eq!(
            test_parse(let_stmt(), &tokens),
            Statement::Let {
                name: test_ident("x"),
                ty: Type::I32,
                value: Expression::Number(42),
            }
        );
    }
}
