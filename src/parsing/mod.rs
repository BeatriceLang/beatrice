use anyhow::Result;
use chumsky::{Parser, input::Input};

use crate::{
    ast::Program,
    lexing::token::Token,
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
mod function;
mod ident;
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
            .map(|token| (token.inner.clone(), token.span.clone().into()))
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
            Ok(program) => self.advance_to(CompilerState::Check(program)),
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
        let mut index = 0;
        [
            $({
                let span = chumsky::span::SimpleSpan::from(index..index + 1);
                index += 1;
                ($token, span)
            }),*
        ]
    }};
}

#[cfg(test)]
pub(crate) use test_tokens;

#[cfg(test)]
pub(crate) fn test_input<'a>(
    tokens: &'a [(Token, chumsky::span::SimpleSpan)],
) -> chumsky::input::MappedInput<
    'a,
    Token,
    chumsky::span::SimpleSpan,
    &'a [(Token, chumsky::span::SimpleSpan)],
> {
    let eoi = tokens
        .last()
        .map(|(_, span)| span.end..span.end)
        .unwrap_or(0..0)
        .into();

    tokens.split_token_span(eoi)
}

#[cfg(test)]
pub(crate) fn test_parse<'a, O>(
    parser: impl Parser<
        'a,
        chumsky::input::MappedInput<
            'a,
            Token,
            chumsky::span::SimpleSpan,
            &'a [(Token, chumsky::span::SimpleSpan)],
        >,
        O,
        chumsky::extra::Err<chumsky::error::Rich<'a, Token, chumsky::span::SimpleSpan>>,
    >,
    tokens: &'a [(Token, chumsky::span::SimpleSpan)],
) -> O {
    parser.parse(test_input(tokens)).unwrap()
}

#[cfg(test)]
pub(crate) fn test_ident(name: &str) -> crate::ast::Ident {
    crate::ast::Ident::new(name.into(), 0..0)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            Block, Function, Program, Type,
            expression::{BinaryOpKind, Expression},
            statement::Statement,
        },
        lexing::token::Token,
        parsing::parser,
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
                functions: vec![Function {
                    name: test_ident("main"),
                    params: vec![],
                    return_type: Type::I32,
                    body: Block {
                        statements: vec![Statement::Return(Expression::Number(42))],
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
                functions: vec![Function {
                    name: test_ident("main"),
                    params: vec![],
                    return_type: Type::I32,
                    body: Block {
                        statements: vec![Statement::Return(Expression::BinaryOp {
                            lhs: Expression::Number(1).into(),
                            kind: BinaryOpKind::Add,
                            rhs: Expression::Number(2).into(),
                        })],
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
                functions: vec![Function {
                    name: test_ident("add"),
                    params: vec![
                        (test_ident("lhs"), Type::I32),
                        (test_ident("rhs"), Type::I32)
                    ],
                    return_type: Type::I32,
                    body: Block {
                        statements: vec![Statement::Return(Expression::Number(42))],
                    },
                }],
            }
        );
    }
}
