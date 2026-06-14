use chumsky::{primitive::choice, select};

use crate::{
    ast::expression::Expression, lexing::token::Token,
    parsing::expr::function_call::function_call_expr,
};

pub fn primary_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let base = select! {
        Token::Number(value) => Expression::Number(value),
        Token::Ident(name) => Expression::Ident(name.clone()),
    };

    choice((function_call_expr(expr), base))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use crate::{
        ast::expression::Expression,
        lexing::token::Token,
        parsing::expr::{expr, primary::primary_expr},
    };

    #[test]
    fn parses_base_expr() {
        let number_tokens = [Token::Number(42)];
        let ident_tokens = [Token::Ident("x".into())];

        assert_eq!(
            primary_expr(expr()).parse(&number_tokens).unwrap(),
            Expression::Number(42)
        );
        assert_eq!(
            primary_expr(expr()).parse(&ident_tokens).unwrap(),
            Expression::Ident("x".into())
        );
    }
}
