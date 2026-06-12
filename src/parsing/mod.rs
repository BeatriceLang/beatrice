use crate::ast::Program;

macro_rules! parser_type {
    ($ret:ty) => {
        impl chumsky::Parser<'a, &'a [$crate::lexing::token::Token], $ret> + Clone
    };
}

mod block;
mod expr;
mod function;
mod ident;
mod program;
mod statement;
mod ty;

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> parser_type!(Program) {
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
    fn parses_return_math_op_function() {
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
