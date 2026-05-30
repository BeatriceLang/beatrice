use chumsky::{IterParser, Parser, prelude::just};

use crate::{ast::Block, lexing::token::Token, parsing::statement::stmt, parsing_rule};

parsing_rule! {
    block -> Block {
        stmt()
            .repeated()
            .collect()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .map(|statements| Block { statements })
    }
}
