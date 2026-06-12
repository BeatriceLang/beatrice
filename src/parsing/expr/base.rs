use chumsky::select;

use crate::{ast::expression::Expression, lexing::token::Token};

pub fn base_expr<'a>() -> parser_type!(Expression) {
    select! {
        Token::Number(value) => Expression::Number(value),
        Token::Ident(name) => Expression::Ident(name.clone()),
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;

    use crate::{
        ast::expression::Expression, lexing::token::Token, parsing::expr::base::base_expr,
    };

    #[test]
    fn parses_base_expr() {
        let number_tokens = [Token::Number(42)];
        let ident_tokens = [Token::Ident("x".into())];

        assert_eq!(
            base_expr().parse(&number_tokens).unwrap(),
            Expression::Number(42)
        );
        assert_eq!(
            base_expr().parse(&ident_tokens).unwrap(),
            Expression::Ident("x".into())
        );
    }
}
