use chumsky::{Parser, primitive::just};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ty::ty};

pub(super) fn cast<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    expr.foldl(just(Token::As).ignore_then(ty()).repeated(), |value, to| {
        Expression::Cast {
            value: Box::new(value),
            to,
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{expression::Expression, ty::Type},
        lexing::token::Token,
        parsing::{
            expr::{atom::atom, expr},
            test_parse, test_tokens,
        },
    };

    #[test]
    fn parses_cast_expr() {
        let tokens = test_tokens![Token::Number(42), Token::As, Token::I32];

        assert_eq!(
            test_parse(atom(expr()), &tokens),
            Expression::Cast {
                value: Expression::Number(42).into(),
                to: Type::I32,
            }
        );
    }

    #[test]
    fn parses_chained_cast_expr() {
        let tokens = test_tokens![
            Token::Number(42),
            Token::As,
            Token::U32,
            Token::As,
            Token::I32,
        ];

        assert_eq!(
            test_parse(atom(expr()), &tokens),
            Expression::Cast {
                value: Expression::Cast {
                    value: Expression::Number(42).into(),
                    to: Type::U32,
                }
                .into(),
                to: Type::I32,
            }
        );
    }
}
