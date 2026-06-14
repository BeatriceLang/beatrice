use chumsky::{
    error::{Rich, RichPattern, RichReason},
    input::Input,
};

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    lexing::token::Token,
};

impl From<Rich<'_, Token>> for Diagnostic {
    fn from(error: Rich<'_, Token>) -> Self {
        let message = match error.reason() {
            RichReason::ExpectedFound { expected, found } => {
                let expected = expected
                    .iter()
                    .map(display_rich_pattern)
                    .collect::<Vec<_>>()
                    .join(", ");
                let found = found
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or("EOF".into());

                format!("Expected `{expected}`, found `{found}`")
            }
            RichReason::Custom(reason) => format!("{reason:?}"),
        };

        Diagnostic {
            span: error.span().into_range(),
            kind: DiagnosticKind::Error,
            label: message.clone(),
            message,
        }
    }
}

fn display_rich_pattern(pattern: &RichPattern<'_, Token>) -> String {
    match pattern {
        RichPattern::Token(token) => token.to_string(),
        RichPattern::Label(label) => label.to_string(),
        RichPattern::Identifier(ident) => ident.clone(),
        RichPattern::SomethingElse => "Something else".into(),
        RichPattern::EndOfInput => "EOF".into(),
        RichPattern::Any => "Any".into(),
        _ => todo!(),
    }
}
