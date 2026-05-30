use chumsky::{IterParser, Parser, prelude::just};

use crate::{
    ast::Block,
    lexing::token::Token,
    parsing::{parsing_rule, statement::stmt},
};

parsing_rule! {
    block -> Block {
        stmt()
            .repeated()
            .collect()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .map(|statements| Block { statements })
    }
}
