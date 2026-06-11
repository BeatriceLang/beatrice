use chumsky::Parser;

use crate::{ast::Program, lexing::token::Token};

mod block;
mod expr;
mod function;
mod ident;
mod program;
mod statement;
mod ty;

macro_rules! parsing_rule {
    {
        $name:ident -> $ret:ty $body:block

        $(
            test $test_body:block
        )?
    } => {
        pub fn $name<'a>() -> impl chumsky::Parser<'a, &'a [$crate::lexing::token::Token], $ret> $body

        $crate::parsing::parsing_rule!(@test $name $($test_body)?);
    };

    (@test $name:ident $test_body:block) => {
        paste::paste! {
            #[cfg(test)]
            #[test]
            fn [<parses_ $name>]() $test_body
        }
    };

    (@test $name:ident) => {};
}

pub(crate) use parsing_rule;

// Parser takes &[Token] as input, outputs a Program
pub fn parser<'a>() -> impl Parser<'a, &'a [Token], Program> {
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
        let tokens: Vec<_> = Token::lexer(input)
            .map(|token| token.unwrap())
            .collect();

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
        let tokens: Vec<_> = Token::lexer(input)
            .map(|token| token.unwrap())
            .collect();

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
