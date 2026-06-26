use chumsky::{primitive::just, IterParser, Parser};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ident::ident};

pub(super) fn create_struct<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let field = ident()
        .then_ignore(just(Token::Colon))
        .then(expr)
        .then_ignore(just(Token::Comma));
    let body = field
        .repeated()
        .collect::<Vec<_>>()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

    just(Token::New)
        .ignore_then(ident())
        .then(body)
        .map(|(name, fields)| Expression::CreateStruct {
            name,
            fields: fields
                .into_iter()
                .map(|(i, expr)| (i, Box::new(expr)))
                .collect(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expression::{BinaryOpKind, Expression},
        parsing::{expr::expr, test_ident, test_parse, test_tokens},
    };

    #[test]
    fn parses_empty_struct_expression() {
        let tokens = test_tokens![
            Token::New,
            Token::Ident("Point".into()),
            Token::LeftBrace,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(create_struct(expr()), &tokens),
            Expression::CreateStruct {
                name: test_ident("Point"),
                fields: vec![],
            }
        );
    }

    #[test]
    fn parses_struct_expression_with_fields() {
        let tokens = test_tokens![
            Token::New,
            Token::Ident("Point".into()),
            Token::LeftBrace,
            Token::Ident("x".into()),
            Token::Colon,
            Token::Number(1),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Colon,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
            Token::Comma,
            Token::RightBrace,
        ];

        assert_eq!(
            test_parse(create_struct(expr()), &tokens),
            Expression::CreateStruct {
                name: test_ident("Point"),
                fields: vec![
                    (test_ident("x"), Expression::Number(1).into()),
                    (
                        test_ident("y"),
                        Expression::BinaryOp {
                            lhs: Expression::Number(2).into(),
                            kind: BinaryOpKind::Add,
                            rhs: Expression::Number(3).into(),
                        }
                        .into(),
                    ),
                ],
            }
        );
    }
}
