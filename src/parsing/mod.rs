use chumsky::prelude::Boxed;

use crate::{ast::Program, lexing::token::Token};

mod block;
mod expr;
mod function;
mod ident;
mod program;
mod statement;
mod ty;

pub type BeatriceParser<'a, T> = Boxed<'a, 'a, &'a [Token], T>;

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> BeatriceParser<'a, Program> {
    program::program()
}

#[cfg(test)]
mod tests {
    use chumsky::Parser as _;
    use logos::Logos;

    use crate::{
        ast::{
            Block, Function, Program, Type,
            expression::{Expression, MathOpKind},
            statement::Statement,
        },
        lexing::token::Token,
        parsing::parser,
    };

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
                    return_type: Type::I32,
                    body: Block {
                        statements: vec![Statement::Return(Expression::Number(42))],
                    },
                }],
            }
        );
    }

    #[test]
    fn parses_return_math_op_function() {
        let input = "fn main() -> i32 { return 1 + 2; }";
        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        let program = parser().parse(&tokens).unwrap();

        assert_eq!(
            program,
            Program {
                functions: vec![Function {
                    name: "main".into(),
                    return_type: Type::I32,
                    body: Block {
                        statements: vec![Statement::Return(Expression::MathOp {
                            lhs: Expression::Number(1).into(),
                            kind: MathOpKind::Add,
                            rhs: Expression::Number(2).into(),
                        })],
                    },
                }],
            }
        );
    }
}
