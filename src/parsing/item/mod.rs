use chumsky::{Parser, primitive::choice};

use crate::{
    ast::Item,
    parsing::item::{extern_fn::extern_function, function::function, import::import},
};

mod extern_fn;
mod function;
mod import;

pub fn item<'a>() -> parser_type!(Item) {
    choice((
        extern_function().map(Item::ExternFunction),
        function().map(Item::Function),
        import(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Type, function::ExternFunction},
        lexing::token::Token,
        parsing::{test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_extern_function_item() {
        let tokens = test_tokens![
            Token::Extern,
            Token::Fn,
            Token::Ident("puts".into()),
            Token::LeftParen,
            Token::Ident("value".into()),
            Token::Colon,
            Token::String,
            Token::RightParen,
            Token::RetArrow,
            Token::I32,
            Token::Semicolon,
        ];

        assert_eq!(
            test_parse(item(), &tokens),
            Item::ExternFunction(ExternFunction {
                name: test_ident("puts"),
                params: vec![(test_ident("value"), Type::String)],
                return_type: Type::I32,
            })
        );
    }
}
