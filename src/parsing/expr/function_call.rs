use chumsky::{IterParser, Parser, prelude::just};

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
    use chumsky::Parser as _;

    use crate::{
        ast::expression::Expression,
        lexing::token::Token,
        parsing::expr::{expr, function_call::function_call_expr},
    };

    #[test]
    fn parses_function_call_expr() {
        let tokens = [
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::RightParen,
        ];

        assert_eq!(
            function_call_expr(expr()).parse(&tokens).unwrap(),
            Expression::FunctionCall {
                name: "test".into(),
                args: vec![]
            }
        );
    }

    #[test]
    fn parses_function_call_expr_with_args() {
        let single_arg_tokens = [
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::Number(42),
            Token::RightParen,
        ];
        let multiple_arg_tokens = [
            Token::Ident("test".into()),
            Token::LeftParen,
            Token::Number(1),
            Token::Comma,
            Token::Ident("x".into()),
            Token::RightParen,
        ];

        assert_eq!(
            function_call_expr(expr())
                .parse(&single_arg_tokens)
                .unwrap(),
            Expression::FunctionCall {
                name: "test".into(),
                args: vec![Expression::Number(42)]
            }
        );
        assert_eq!(
            function_call_expr(expr())
                .parse(&multiple_arg_tokens)
                .unwrap(),
            Expression::FunctionCall {
                name: "test".into(),
                args: vec![Expression::Number(1), Expression::Ident("x".into())]
            }
        );
    }
}
