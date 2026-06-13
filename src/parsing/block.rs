use chumsky::{IterParser, Parser, prelude::just, recursive::recursive};

use crate::{ast::Block, lexing::token::Token, parsing::statement::stmt};

pub fn block<'a>() -> parser_type!(Block) {
    recursive(|block| {
        stmt(block)
            .repeated()
            .collect()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .map(|statements| Block { statements })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_block() {
        use chumsky::Parser as _;

        let tokens = [
            Token::LeftBrace,
            Token::Return,
            Token::Number(42),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(
            block().parse(&tokens).unwrap(),
            Block {
                statements: vec![crate::ast::statement::Statement::Return(
                    crate::ast::expression::Expression::Number(42)
                )],
            }
        );
    }
}
