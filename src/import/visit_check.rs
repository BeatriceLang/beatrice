use std::path::PathBuf;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    import::ImportProcessor,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum VisitState {
    Process,
    Skip,
}

impl<'a> ImportProcessor<'a> {
    #[must_use]
    pub(super) fn visit_state(&mut self, path: &PathBuf) -> VisitState {
        if self.visited.contains(path) {
            VisitState::Skip
        } else if self.visiting.contains(path) {
            self.push_cycle_diagnostic(path);
            VisitState::Skip
        } else {
            VisitState::Process
        }
    }

    fn push_cycle_diagnostic(&mut self, path: &PathBuf) {
        let diag = Diagnostic {
            // TODO: import items should carry spans so this can point at the import path.
            span: 0..0,
            kind: DiagnosticKind::Error,
            label: format!(
                "{path:?} is importing [another path] while [another path] is importing {path:?}"
            ),
            message: "Circular import".into(),
        };

        self.diagnostics.push(diag);
    }
}
