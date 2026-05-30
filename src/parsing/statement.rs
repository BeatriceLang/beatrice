use chumsky::{
    Parser,
    extra::State,
    prelude::{choice, just},
};

use crate::{ast::statement::Statement, lexing::token::Token, parsing::expr::expr, parsing_rule};

parsing_rule! {
    return_stmt -> Statement {
        just(Token::Return)
            .ignore_then(expr())
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Return)
    }
}

parsing_rule! {
    expr_stmt -> Statement {
        expr()
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Expression)
    }
}

parsing_rule! {
    stmt -> Statement {
        choice((
                return_stmt(),
                expr_stmt()
        ))
    }
}
