use chumsky::{prelude::just, IterParser, Parser};

use crate::{ast::expression::Expression, lexing::token::Token, parsing::ident::ident};

pub fn function_call_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    ident()
        .then_ignore(just(Token::LeftParen))
        .then(expr.separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::RightParen))
        .map(|(name, args)| Expression::FunctionCall { name, args })
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::expression::Expression,
        lexing::token::Token,
        parsing::{
            expr::{expr, function_call::function_call_expr},
            test_ident, test_parse, test_tokens,
        },
    };

    #[test]
    fn parses_function_call_expr() {
        let tokens = test_tokens![
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::RightParen,
        ];

        assert_eq!(
            test_parse(function_call_expr(expr()), &tokens),
            Expression::FunctionCall {
                name: test_ident("test"),
                args: vec![]
            }
        );
    }

    #[test]
    fn parses_function_call_expr_with_args() {
        let single_arg_tokens = test_tokens![
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::Number(42),
            Token::RightParen,
        ];
        let multiple_arg_tokens = test_tokens![
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::Number(1),
            Token::Comma,
            Token::Ident("x".into()),
            Token::RightParen,
        ];

        assert_eq!(
            test_parse(function_call_expr(expr()), &single_arg_tokens),
            Expression::FunctionCall {
                name: test_ident("test"),
                args: vec![Expression::Number(42)]
            }
        );
        assert_eq!(
            test_parse(function_call_expr(expr()), &multiple_arg_tokens),
            Expression::FunctionCall {
                name: test_ident("test"),
                args: vec![Expression::Number(1), Expression::Ident(test_ident("x"))]
            }
        );
    }
}
