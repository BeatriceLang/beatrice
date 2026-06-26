use chumsky::{Parser, primitive::just};

use crate::{
    ast::{Block, statement::Statement},
    lexing::token::Token,
    parsing::expr::expr,
};

pub(super) fn while_stmt<'a>(block: parser_type!(Block)) -> parser_type!(Statement) {
    just(Token::While)
        .ignore_then(expr())
        .then(block)
        .map(|(cond, body)| Statement::While { cond, body })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            Block,
            expression::{BinaryOpKind, Expression},
            statement::Statement,
        },
        parsing::{block::block, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_while_stmt() {
        let tokens = test_tokens![
            Token::While,
            Token::Ident("n".into()),
            Token::LessThan,
            Token::Number(10),
            Token::LeftBrace,
            Token::Ident("n".into()),
            Token::Assign,
            Token::Ident("n".into()),
            Token::Add,
            Token::Number(1),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(while_stmt(block()), &tokens),
            Statement::While {
                cond: Expression::BinaryOp {
                    lhs: Expression::Ident(test_ident("n")).into(),
                    kind: BinaryOpKind::LessThan,
                    rhs: Expression::Number(10).into(),
                },
                body: Block {
                    statements: vec![Statement::Assign {
                        ident: test_ident("n"),
                        value: Expression::BinaryOp {
                            lhs: Expression::Ident(test_ident("n")).into(),
                            kind: BinaryOpKind::Add,
                            rhs: Expression::Number(1).into(),
                        },
                    }],
                },
            }
        );
    }
}
