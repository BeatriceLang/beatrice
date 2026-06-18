use chumsky::{
    error::{Rich, RichPattern, RichReason},
    span::SimpleSpan,
};

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    lexing::token::Token,
};

impl From<Rich<'_, Token, SimpleSpan>> for Diagnostic {
    fn from(error: Rich<'_, Token, SimpleSpan>) -> Self {
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
        _ => todo!("Handle unknown rich pattern"),
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{
        Parser,
        error::{Rich, RichPattern},
        prelude::just,
        span::SimpleSpan,
    };

    use crate::{
        diagnostic::{Diagnostic, DiagnosticKind},
        lexing::token::Token,
        parsing::{test_input, test_tokens},
    };

    use super::display_rich_pattern;

    #[test]
    fn displays_token_patterns_with_token_display() {
        let pattern = RichPattern::Token(Token::Ident("name".into()).into());

        assert_eq!(display_rich_pattern(&pattern), "name");
    }

    #[test]
    fn displays_end_of_input_as_eof() {
        assert_eq!(display_rich_pattern(&RichPattern::EndOfInput), "EOF");
    }

    #[test]
    fn converts_custom_rich_error_to_diagnostic() {
        let error = Rich::<Token, SimpleSpan>::custom(
            SimpleSpan::from(2..5),
            "expected function declaration",
        );
        let diagnostic = Diagnostic::from(error);

        assert_eq!(diagnostic.span, 2..5);
        assert_eq!(diagnostic.kind, DiagnosticKind::Error);
        assert_eq!(diagnostic.message, "\"expected function declaration\"");
        assert_eq!(diagnostic.label, "\"expected function declaration\"");
    }

    #[test]
    fn converts_expected_found_error_to_diagnostic() {
        let tokens = test_tokens![Token::Ident("fasdf".into())];
        let error = just::<_, _, chumsky::extra::Err<Rich<Token, SimpleSpan>>>(Token::Fn)
            .parse(test_input(&tokens))
            .into_errors()
            .remove(0);

        let diagnostic = Diagnostic::from(error);

        assert_eq!(diagnostic.span, 0..1);
        assert_eq!(diagnostic.kind, DiagnosticKind::Error);
        assert_eq!(diagnostic.message, "Expected `fn`, found `fasdf`");
        assert_eq!(diagnostic.label, "Expected `fn`, found `fasdf`");
    }

    #[test]
    fn converts_missing_token_error_to_eof_diagnostic() {
        let tokens = test_tokens![];
        let error = just::<_, _, chumsky::extra::Err<Rich<Token, SimpleSpan>>>(Token::Fn)
            .parse(test_input(&tokens))
            .into_errors()
            .remove(0);

        let diagnostic = Diagnostic::from(error);

        assert_eq!(diagnostic.span, 0..0);
        assert_eq!(diagnostic.message, "Expected `fn`, found `EOF`");
        assert_eq!(diagnostic.label, "Expected `fn`, found `EOF`");
    }
}
