use std::{ops::Range, path::PathBuf, process::exit};

use anyhow::{Context, Result};
use ariadne::{Label, Report, ReportKind, Source};

#[derive(Debug)]
pub struct Diagnostic {
    pub span: Range<usize>,
    pub kind: DiagnosticKind,
    pub message: String,
    pub label: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

impl<'a> From<DiagnosticKind> for ReportKind<'a> {
    fn from(val: DiagnosticKind) -> Self {
        match val {
            DiagnosticKind::Error => ReportKind::Error,
            DiagnosticKind::Warning => ReportKind::Warning,
        }
    }
}

type AriadneSpan = (String, Range<usize>);

impl Diagnostic {
    fn print(&self, source: String, source_path: PathBuf) -> Result<()> {
        let span = self.ariadne_span(source_path.clone())?;

        Report::build(self.kind.into(), span.clone())
            .with_message(self.message.clone())
            .with_label(Label::new(span).with_message(self.label.clone()))
            .finish()
            .eprint((self.source_file_name(source_path)?, Source::from(source)))?;
        Ok(())
    }

    fn ariadne_span(&self, source_path: PathBuf) -> Result<AriadneSpan> {
        Ok((self.source_file_name(source_path)?, self.span.clone()))
    }

    fn source_file_name(&self, source_path: PathBuf) -> Result<String> {
        Ok(source_path
            .file_name()
            .context("Failed to parse source file name")?
            .to_str()
            .context("Failed to parse source file name")?
            .to_string())
    }
}

#[derive(Debug)]
pub struct Diagnostics {
    inner: Vec<Diagnostic>,
    source: String,
    source_path: PathBuf,
}

impl Diagnostics {
    pub fn new(source: String, source_path: PathBuf) -> Self {
        Self {
            inner: vec![],
            source,
            source_path,
        }
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.inner.push(diagnostic);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.inner.iter()
    }

    /// Processes the diagnostics.
    ///
    /// Exits if there is any diagnostic.
    pub fn process(&self) -> Result<()> {
        if !self.inner.is_empty() {
            for diagnostic in &self.inner {
                diagnostic
                    .print(self.source.clone(), self.source_path.clone())
                    .context("Printing diagnostic failed")?;
            }

            exit(1);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{Diagnostic, DiagnosticKind, Diagnostics};

    fn diagnostic() -> Diagnostic {
        Diagnostic {
            span: 3..4,
            kind: DiagnosticKind::Error,
            message: "Unexpected token".into(),
            label: "Unexpected character `@`".into(),
        }
    }

    #[test]
    fn diagnostic_ariadne_span_uses_source_file_name() {
        let diagnostic = diagnostic();

        let span = diagnostic
            .ariadne_span(PathBuf::from("/tmp/main.bea"))
            .unwrap();

        assert_eq!(span, ("main.bea".into(), 3..4));
    }

    #[test]
    fn diagnostic_source_file_name_errors_without_file_name() {
        let diagnostic = diagnostic();

        let error = diagnostic.source_file_name(PathBuf::from("/")).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Failed to parse source file name")
        );
    }

    #[test]
    fn diagnostics_new_starts_empty() {
        let diagnostics = Diagnostics::new("fn main() -> i32 {}".into(), "main.bea".into());

        assert!(diagnostics.inner.is_empty());
    }

    #[test]
    fn diagnostics_push_records_diagnostic() {
        let mut diagnostics = Diagnostics::new("@".into(), "main.bea".into());

        diagnostics.push(diagnostic());

        assert_eq!(diagnostics.inner.len(), 1);
    }

    #[test]
    fn diagnostics_process_empty_diagnostics_succeeds() {
        let diagnostics = Diagnostics::new("fn main() -> i32 {}".into(), "main.bea".into());

        diagnostics.process().unwrap();
    }
}
