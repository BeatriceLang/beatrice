use chumsky::select;

use crate::{ast::expression::Expression, lexing::token::Token, parsing::parsing_rule};

parsing_rule! {
    expr -> Expression {
        select! {
            Token::Number(value) => Expression::Number(value),
            Token::Ident(name) => Expression::Ident(name.clone()),
        }
    }
}
