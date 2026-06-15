use chumsky::{Parser, primitive::choice, select};

use crate::{
    ast::expression::Expression,
    lexing::token::Token,
    parsing::{expr::function_call::function_call_expr, ident::ident},
};

pub fn primary_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let base = select! {
        Token::Number(value) => Expression::Number(value),
    }
    .or(ident().map(Expression::Ident));

    choice((function_call_expr(expr), base))
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::expression::Expression,
        lexing::token::Token,
        parsing::{
            expr::{expr, primary::primary_expr},
            test_parse, test_tokens,
        },
    };

    #[test]
    fn parses_base_expr() {
        let number_tokens = test_tokens![Token::Number(42)];
        let ident_tokens = test_tokens![Token::Ident("x".into())];

        assert_eq!(
            test_parse(primary_expr(expr()), &number_tokens),
            Expression::Number(42)
        );
        assert_eq!(
            test_parse(primary_expr(expr()), &ident_tokens),
            Expression::Ident(test_ident("x"))
        );
    }
}
