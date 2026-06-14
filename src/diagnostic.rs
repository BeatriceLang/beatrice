use std::{ops::Range, path::PathBuf, process::exit};

use anyhow::{Context, Result};
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{select, span::SpanWrap};
use inkwell::AddressSpace;

#[derive(Debug)]
pub struct Diagnostic {
    pub span: Range<usize>,
    pub kind: DiagnosticKind,
    pub message: String,
    pub label: String,
}

#[derive(Clone, Copy, Debug)]
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
