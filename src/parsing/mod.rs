use anyhow::Result;
use chumsky::Parser;

use crate::{
    ast::Program,
    state::{Compiler, CompilerState},
};

macro_rules! parser_type {
    ($ret:ty) => {
        impl chumsky::Parser<'a, &'a [$crate::lexing::token::Token], $ret,
            chumsky::extra::Err<chumsky::error::Rich<'a, $crate::lexing::token::Token>>> + Clone
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

        let parse_result = parser().parse(tokens).into_result();

        match parse_result {
            Ok(program) => self.advance_to(CompilerState::Codegen(program)),
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

    #[test]
    fn parses_return_number_function() {
        let input = "fn main() -> i32 { return 42; }";
        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        let program = parser().parse(&tokens).unwrap();

        assert_eq!(
            program,
            Program {
                functions: vec![Function {
                    name: "main".into(),
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
        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        let program = parser().parse(&tokens).unwrap();

        assert_eq!(
            program,
            Program {
                functions: vec![Function {
                    name: "main".into(),
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
        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        let program = parser().parse(&tokens).unwrap();

        assert_eq!(
            program,
            Program {
                functions: vec![Function {
                    name: "add".into(),
                    params: vec![("lhs".into(), Type::I32), ("rhs".into(), Type::I32)],
                    return_type: Type::I32,
                    body: Block {
                        statements: vec![Statement::Return(Expression::Number(42))],
                    },
                }],
            }
        );
    }
}
