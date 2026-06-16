use std::path::PathBuf;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind, Diagnostics},
    import::ImportProcessor,
};

impl<'a> ImportProcessor<'a> {
    #[must_use]
    pub(super) fn visit_check(&mut self, path: PathBuf) -> bool {
        if self.visited.contains(&path) {
            self.push_diagnostic(path);
            true
        } else {
            self.visited.push(path);
            false
        }
    }

    fn push_diagnostic(&mut self, path: PathBuf) {
        let diag = Diagnostic {
            // TODO
            span: 0..0,
            kind: DiagnosticKind::Error,
            message: format!("Duplicate import of {path:?}"),
            label: format!("{path:?} have already been imported before"),
        };

        self.diagnostics.push(diag);
    }
}
