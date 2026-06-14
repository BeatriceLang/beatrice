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

                format!("Expected `{expected}`, found `{found:?}`")
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
        RichPattern::Token(token) => format!("{:?}", token),
        RichPattern::Label(label) => label.to_string(),
        RichPattern::Identifier(ident) => ident.clone(),
        RichPattern::SomethingElse => "something else".into(),
        RichPattern::EndOfInput => "end of input".into(),
        RichPattern::Any => "any".into(),
        _ => todo!(),
    }
}
