use chumsky::{
    Parser,
    prelude::{choice, just},
};

use crate::{
    ast::statement::Statement,
    lexing::token::Token,
    parsing::{expr::expr, parsing_rule},
};

parsing_rule! {
    return_stmt -> Statement {
        just(Token::Return)
            .ignore_then(expr())
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Return)
    }

    test {
        use chumsky::Parser as _;

        let tokens = [Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            return_stmt().parse(&tokens).unwrap(),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }
}

parsing_rule! {
    expr_stmt -> Statement {
        expr()
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Expression)
    }

    test {
        use chumsky::Parser as _;

        let tokens = [Token::Number(42), Token::Semicolon];

        assert_eq!(
            expr_stmt().parse(&tokens).unwrap(),
            Statement::Expression(crate::ast::expression::Expression::Number(42))
        );
    }
}

parsing_rule! {
    stmt -> Statement {
        choice((
                return_stmt(),
                expr_stmt()
        ))
    }

    test {
        use chumsky::Parser as _;

        let tokens = [Token::Return, Token::Number(42), Token::Semicolon];

        assert_eq!(
            stmt().parse(&tokens).unwrap(),
            Statement::Return(crate::ast::expression::Expression::Number(42))
        );
    }
}
