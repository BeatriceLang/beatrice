use anyhow::Result;
use logos::Logos;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    lexing::token::Token,
    state::{Compiler, CompilerState},
};

pub mod token;

impl Compiler {
    pub fn lex(&mut self) -> Result<()> {
        let CompilerState::Lex(source) = &self.state else {
            panic!("Unexpected compiler state")
        };

        let mut tokens = vec![];

        for (token, span) in Token::lexer(source).clone().spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(_) => {
                    let char = source[span.clone()].to_string();

                    self.diagnostics.push(Diagnostic {
                        span,
                        kind: DiagnosticKind::Error,
                        message: "Unexpected token".into(),
                        label: format!("Unexpected character `{char}`"),
                    });
                }
            }
        }

        self.advance_to(CompilerState::Parse(tokens))
    }
}
