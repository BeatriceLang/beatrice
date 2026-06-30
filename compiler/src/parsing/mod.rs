use anyhow::Result;
use chumsky::{Parser, input::Input, span::SimpleSpan};

use crate::{
    ast::Program,
    state::{Compiler, CompilerState},
};

macro_rules! parser_type {
    ($ret:ty) => {
        impl chumsky::Parser<'a, chumsky::input::MappedInput<'a, $crate::lexing::token::Token, chumsky::span::SimpleSpan, &'a [($crate::lexing::token::Token, chumsky::span::SimpleSpan)]>, $ret,
            chumsky::extra::Err<chumsky::error::Rich<'a, $crate::lexing::token::Token, chumsky::span::SimpleSpan>>> + Clone
    };
}

mod block;
mod diagnostic;
mod expr;
mod ident;
mod item;
mod program;
mod statement;
mod ty;

impl Compiler {
    pub fn parse(&mut self) -> Result<()> {
        let CompilerState::Parse(tokens) = &self.state else {
            panic!("Unexpected compiler state")
        };

        let tokens = tokens
            .iter()
            .map(|token| {
                (
                    token.inner.clone(),
                    SimpleSpan::from(token.span.start..token.span.end),
                )
            })
            .collect::<Vec<_>>();
        let eoi = tokens
            .last()
            .map(|(_, span)| span.end..span.end)
            .unwrap_or(0..0)
            .into();

        let parse_result = parser()
            .parse(tokens.as_slice().split_token_span(eoi))
            .into_result();

        match parse_result {
            Ok(program) => self.advance_to(CompilerState::Import(program)),
            Err(errors) => {
                for error in errors {
                    self.diagnostics.push(error.into());
                }

                self.advance_to(CompilerState::Error)
            }
        }
    }
}

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> parser_type!(Program) {
    program::program()
}

#[cfg(test)]
macro_rules! test_tokens {
    ($($token:expr),* $(,)?) => {{
        vec![$($token),*]
            .into_iter()
            .enumerate()
            .map(|(index, token)| {
                let span = chumsky::span::SimpleSpan::from(index..index + 1);
                (token, span)
            })
            .collect::<Vec<_>>()
    }};
}

#[cfg(test)]
pub(crate) use test_tokens;

#[cfg(test)]
pub fn test_input(
    tokens: &[(crate::lexing::token::Token, SimpleSpan)],
) -> chumsky::input::MappedInput<
    '_,
    crate::lexing::token::Token,
    SimpleSpan,
    &[(crate::lexing::token::Token, SimpleSpan)],
> {
    let eoi = tokens
        .last()
        .map(|(_, span)| span.end..span.end)
        .unwrap_or(0..0)
        .into();

    tokens.split_token_span(eoi)
}

#[cfg(test)]
pub fn test_parse<'a, O>(
    parser: impl Parser<
        'a,
        chumsky::input::MappedInput<
            'a,
            crate::lexing::token::Token,
            SimpleSpan,
            &'a [(crate::lexing::token::Token, SimpleSpan)],
        >,
        O,
        chumsky::extra::Err<chumsky::error::Rich<'a, crate::lexing::token::Token, SimpleSpan>>,
    >,
    tokens: &'a [(crate::lexing::token::Token, SimpleSpan)],
) -> O {
    parser.parse(test_input(tokens)).unwrap()
}

#[cfg(test)]
pub fn test_ident(name: &str) -> crate::ast::ident::Ident {
    crate::ast::ident::Ident::new(name.into(), 0..0)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            Block, Program,
            expression::{BinaryOpKind, Expression},
            item::Item,
            statement::Statement,
            ty::Type,
        },
        lexing::token::Token,
        parsing::{parser, test_ident, test_input},
    };
    use chumsky::Parser as _;
    use logos::Logos;

    fn lex(input: &str) -> Vec<(Token, chumsky::span::SimpleSpan)> {
        Token::lexer(input)
            .spanned()
            .map(|(token, span)| (token.unwrap(), span.into()))
            .collect()
    }

    #[test]
    fn parses_return_number_function() {
        let input = "fn main() -> i32 { return 42; }";
        let tokens = lex(input);

        let program = parser().parse(test_input(&tokens)).unwrap();

        assert_eq!(
            program,
            Program {
                items: vec![Item::Function {
                    name: test_ident("main"),
                    params: vec![],
                    return_type: Some(Type::I32),
                    body: Block {
                        statements: vec![Statement::Return(Some(Expression::Number(42)))],
                    },
                }],
            }
        );
    }

    #[test]
    fn parses_return_binary_op_function() {
        let input = "fn main() -> i32 { return 1 + 2; }";
        let tokens = lex(input);

        let program = parser().parse(test_input(&tokens)).unwrap();

        assert_eq!(
            program,
            Program {
                items: vec![Item::Function {
                    name: test_ident("main"),
                    params: vec![],
                    return_type: Some(Type::I32),
                    body: Block {
                        statements: vec![Statement::Return(Some(Expression::BinaryOp {
                            lhs: Expression::Number(1).into(),
                            kind: BinaryOpKind::Add,
                            rhs: Expression::Number(2).into(),
                        }))],
                    },
                }],
            }
        );
    }

    #[test]
    fn parses_function_params() {
        let input = "fn add(lhs: i32, rhs: i32) -> i32 { return 42; }";
        let tokens = lex(input);

        let program = parser().parse(test_input(&tokens)).unwrap();

        assert_eq!(
            program,
            Program {
                items: vec![Item::Function {
                    name: test_ident("add"),
                    params: vec![
                        (test_ident("lhs"), Type::I32),
                        (test_ident("rhs"), Type::I32)
                    ],
                    return_type: Some(Type::I32),
                    body: Block {
                        statements: vec![Statement::Return(Some(Expression::Number(42)))],
                    },
                }],
            }
        );
    }
}
