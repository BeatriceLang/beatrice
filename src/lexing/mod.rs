use anyhow::Result;
use logos::Logos;
use tap::Tap;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind, Diagnostics},
    lexing::token::Token,
    span::Spanned,
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

fn lex_inner(source: &str, diagnostics: &mut Diagnostics) -> Vec<Spanned<Token>> {
    vec![].tap_mut(|tokens| {
        for (token, span) in Token::lexer(source).spanned() {
            match token {
                Ok(token) => tokens.push(Spanned::new(token, span)),
                Err(_) => {
                    let char = source[span.clone()].to_string();
                    let message = format!("Unknown character `{char}`");

                    diagnostics.push(Diagnostic {
                        span,
                        kind: DiagnosticKind::Error,
                        label: message.clone(),
                        message,
                    });
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        diagnostic::{DiagnosticKind, Diagnostics},
        lexing::token::Token,
        span::Spanned,
    };

    use super::lex_inner;

    #[test]
    fn lex_source_collects_unexpected_character_diagnostic() {
        let source = "fn @";
        let mut diagnostics = Diagnostics::new(source.into(), PathBuf::from("main.bea"));

        let tokens = lex_inner(source, &mut diagnostics);
        let diagnostic = diagnostics.inner.first().unwrap();

        assert_eq!(tokens, vec![Spanned::new(Token::Fn, 0..2)]);
        assert_eq!(diagnostic.span, 3..4);
        assert_eq!(diagnostic.kind, DiagnosticKind::Error);
        assert_eq!(diagnostic.message, "Unknown character `@`");
        assert_eq!(diagnostic.label, "Unknown character `@`");
    }

    #[test]
    fn lex_source_continues_after_unexpected_character() {
        let source = "fn @ main";
        let mut diagnostics = Diagnostics::new(source.into(), PathBuf::from("main.bea"));

        let tokens = lex_inner(source, &mut diagnostics);

        assert_eq!(
            tokens,
            vec![
                Spanned::new(Token::Fn, 0..2),
                Spanned::new(Token::Ident("main".into()), 5..9),
            ]
        );
        assert_eq!(diagnostics.inner.len(), 1);
    }
}
