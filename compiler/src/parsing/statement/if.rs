use chumsky::{prelude::just, Parser};

use crate::{
    ast::{statement::Statement, Block},
    lexing::token::Token,
    parsing::expr::expr,
};

pub(super) fn if_stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    just(Token::If)
        .ignore_then(expr())
        .then(block)
        .map(|(cond, body)| Statement::If { cond, body })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            expression::{BinaryOpKind, Expression},
            statement::Statement,
            Block,
        },
        parsing::{block::block, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_if_stmt() {
        let tokens = test_tokens![
            Token::If,
            Token::Ident("n".into()),
            Token::LessThan,
            Token::Number(2),
            Token::LeftBrace,
            Token::Return,
            Token::Ident("n".into()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(if_stmt(block()), &tokens),
            Statement::If {
                cond: Expression::BinaryOp {
                    lhs: Expression::Ident(test_ident("n")).into(),
                    kind: BinaryOpKind::LessThan,
                    rhs: Expression::Number(2).into(),
                },
                body: Block {
                    statements: vec![Statement::Return(Some(Expression::Ident(test_ident("n"))))],
                },
            }
        );
    }
}
