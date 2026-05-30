use chumsky::{
    IterParser, Parser,
    prelude::{choice, end, just},
    primitive::select,
    select,
};

use crate::{
    ast::{Block, Function, Program, Type, expresion::Expression, statement::Statement},
    lexing::token::Token,
};

macro_rules! grammar {
    {
        $name:ident -> $ret:ty $body:block
    } => {
        fn $name<'a>() -> impl chumsky::Parser<'a, &'a [$crate::lexing::token::Token], $ret> $body
    };
}

grammar! {
    ident -> String {
        select! {
            Token::Ident(name) => name
        }
    }
}

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> impl Parser<'a, &'a [Token], Program> {
    // Rule for matching ident
    let expr = select! {
        Token::Number(value) => Expression::Number(value),
        Token::Ident(name) => Expression::Ident(name.clone()),
    };

    let ty = select! {
        Token::I32 => Type::I32
    };

    let return_stmt = just(Token::Return)
        .ignore_then(expr)
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Return);

    let expr_stmt = expr
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Expression);

    let stmt = choice((return_stmt, expr_stmt));

    let block = stmt
        .repeated()
        .collect()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
        .map(|statements| Block { statements });

    let function = just(Token::Fn)
        .ignore_then(ident())
        .then_ignore(just(Token::LeftParen))
        .then_ignore(just(Token::RightParen))
        .then_ignore(just(Token::RetArrow))
        .then(ty)
        .then(block)
        .map(|((name, return_type), body)| Function {
            name,
            return_type,
            body,
        });

    function
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| Program { functions })
}
