use chumsky::{Parser, primitive::just};

use crate::{
    ast::statement::Statement,
    lexing::token::Token,
    parsing::{expr::expr, ident::ident},
};

pub(super) fn assign<'a>() -> parser_type!(Statement) {
    ident()
        .then_ignore(just(Token::Assign))
        .then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(|(ident, value)| Statement::Assign { ident, value })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{expression::Expression, statement::Statement},
        parsing::{test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_assign_stmt() {
        let tokens = test_tokens![
            Token::Ident("x".into()),
            Token::Assign,
            Token::Number(42),
            Token::Semicolon,
        ];

        assert_eq!(
            test_parse(assign(), &tokens),
            Statement::Assign {
                ident: test_ident("x"),
                value: Expression::Number(42),
            }
        );
    }
}
