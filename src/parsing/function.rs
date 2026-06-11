use chumsky::{Parser, prelude::just};

use crate::{
    ast::Function,
    lexing::token::Token,
    parsing::{
        block::block,
        ident::ident,
        parsing_rule,
        ty::ty,
    },
};

parsing_rule! {
    function -> Function {
        just(Token::Fn)
            .ignore_then(ident())
            .then_ignore(just(Token::LeftParen))
            .then_ignore(just(Token::RightParen))
            .then_ignore(just(Token::RetArrow))
            .then(ty())
            .then(block())
            .map(|((name, return_type), body)| Function {
                name,
                return_type,
                body,
            })
    }

    test {
        use chumsky::Parser as _;

        let tokens = [
            Token::Fn,
            Token::Ident("main".into()),
            Token::LeftParen,
            Token::RightParen,
            Token::RetArrow,
            Token::I32,
            Token::LeftBrace,
            Token::Return,
            Token::Number(42),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(
            function().parse(&tokens).unwrap(),
            Function {
                name: "main".into(),
                return_type: crate::ast::Type::I32,
                body: crate::ast::Block {
                    statements: vec![crate::ast::statement::Statement::Return(
                        crate::ast::expression::Expression::Number(42)
                    )],
                },
            }
        );
    }
}
