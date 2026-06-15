use chumsky::{Parser, primitive::just};

use crate::{
    ast::{Item, expression::Expression},
    lexing::token::Token,
    parsing::expr::expr,
};

pub(super) fn import<'a>() -> parser_type!(Item) {
    just(Token::Import)
        .ignore_then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(|expr| {
            let Expression::StringLiteral(path) = expr else {
                todo!("Handle error")
            };

            Item::Import(path.into())
        })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::parsing::{test_parse, test_tokens};

    #[test]
    fn parses_import_item() {
        let tokens = test_tokens![
            Token::Import,
            Token::StringLiteral("a.bt".into()),
            Token::Semicolon
        ];

        assert_eq!(
            test_parse(import(), &tokens),
            Item::Import(PathBuf::from("a.bt"))
        );
    }
}
