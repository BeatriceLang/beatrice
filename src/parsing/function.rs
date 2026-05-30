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
}
