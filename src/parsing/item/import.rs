use chumsky::{Parser, primitive::just, select};

use crate::{ast::Item, lexing::token::Token};

pub(super) fn import<'a>() -> parser_type!(Item) {
    just(Token::Import)
        .ignore_then(select! {
            Token::StringLiteral(path) => path.clone(),
        })
        .then_ignore(just(Token::Semicolon))
        .map(|path| Item::Import(path.into()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::parsing::{test_input, test_parse, test_tokens};

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

    #[test]
    fn rejects_non_string_import_path() {
        let tokens = test_tokens![Token::Import, Token::Number(123), Token::Semicolon];
        let errors = import().parse(test_input(&tokens)).into_errors();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].span().into_range(), 1..2);
    }
}
