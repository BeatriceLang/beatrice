use anyhow::Result;
use logos::Logos;
use tap::Tap;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind, Diagnostics},
    lexing::token::Token,
    state::{Compiler, CompilerState},
};

pub mod token;

impl Compiler {
    pub fn lex(&mut self) -> Result<()> {
        let CompilerState::Lex(source) = &self.state else {
            panic!("Unexpected compiler state")
        };

        let tokens = lex_inner(source, &mut self.diagnostics);

        self.advance_to(CompilerState::Parse(tokens))
    }
}

fn lex_inner(source: &str, diagnostics: &mut Diagnostics) -> Vec<Token> {
    vec![].tap_mut(|tokens| {
        for (token, span) in Token::lexer(source).spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(_) => {
                    let char = source[span.clone()].to_string();

                    diagnostics.push(Diagnostic {
                        span,
                        kind: DiagnosticKind::Error,
                        message: "Unexpected token".into(),
                        label: format!("Unexpected character `{char}`"),
                    });
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::diagnostic::{DiagnosticKind, Diagnostics};

    use super::{Token, lex_inner};

    #[test]
    fn lex_source_collects_unexpected_character_diagnostic() {
        let source = "fn @";
        let mut diagnostics = Diagnostics::new(source.into(), PathBuf::from("main.bea"));

        let tokens = lex_inner(source, &mut diagnostics);
        let diagnostic = diagnostics.iter().next().unwrap();

        assert_eq!(tokens, vec![Token::Fn]);
        assert_eq!(diagnostic.span, 3..4);
        assert_eq!(diagnostic.kind, DiagnosticKind::Error);
        assert_eq!(diagnostic.message, "Unexpected token");
        assert_eq!(diagnostic.label, "Unexpected character `@`");
    }

    #[test]
    fn lex_source_continues_after_unexpected_character() {
        let source = "fn @ main";
        let mut diagnostics = Diagnostics::new(source.into(), PathBuf::from("main.bea"));

        let tokens = lex_inner(source, &mut diagnostics);

        assert_eq!(tokens, vec![Token::Fn, Token::Ident("main".into())]);
        assert_eq!(diagnostics.iter().count(), 1);
    }
}
