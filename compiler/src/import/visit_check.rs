use std::path::Path;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    import::ImportProcessor,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum VisitState {
    Process,
    Skip,
}

impl ImportProcessor<'_> {
    #[must_use]
    pub(super) fn visit_state(&mut self, path: &Path) -> VisitState {
        if self.visited.contains(path) {
            VisitState::Skip
        } else if self.visiting.iter().any(|visiting| visiting == path) {
            self.push_cycle_diagnostic(path);
            VisitState::Skip
        } else {
            VisitState::Process
        }
    }

    fn push_cycle_diagnostic(&mut self, path: &Path) {
        let diag = Diagnostic {
            // TODO: import items should carry spans so this can point at the import path.
            span: 0..0,
            kind: DiagnosticKind::Error,
            label: format!(
                "{} is importing [another path] while [another path] is importing {}",
                path.display(),
                path.display()
            ),
            message: "Circular import".into(),
        };

        self.diagnostics.push(diag);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ast::Program,
        diagnostic::{DiagnosticKind, Diagnostics},
        import::{ImportProcessor, VisitState},
    };

    fn diagnostics() -> Diagnostics {
        Diagnostics::new(String::new(), PathBuf::from("main.bt"))
    }

    fn source_path() -> PathBuf {
        PathBuf::from("main.bt")
    }

    #[test]
    fn visit_state_processes_unseen_path() {
        let mut program = Program { items: vec![] };
        let mut diagnostics = diagnostics();
        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        assert_eq!(
            processor.visit_state(&PathBuf::from("new.bt")),
            VisitState::Process
        );
        assert!(diagnostics.inner.is_empty());
    }

    #[test]
    fn visit_state_skips_visited_path() {
        let mut program = Program { items: vec![] };
        let mut diagnostics = diagnostics();
        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        processor.visited.insert(PathBuf::from("visited.bt"));

        assert_eq!(
            processor.visit_state(&PathBuf::from("visited.bt")),
            VisitState::Skip
        );
        assert!(diagnostics.inner.is_empty());
    }

    #[test]
    fn visit_state_skips_visiting_path_and_reports_cycle() {
        let mut program = Program { items: vec![] };
        let mut diagnostics = diagnostics();
        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        processor.visiting.push(PathBuf::from("cycle.bt"));

        assert_eq!(
            processor.visit_state(&PathBuf::from("cycle.bt")),
            VisitState::Skip
        );
        assert_eq!(diagnostics.inner.len(), 1);
        assert_eq!(diagnostics.inner[0].kind, DiagnosticKind::Error);
        assert_eq!(diagnostics.inner[0].message, "Circular import");
        assert_eq!(diagnostics.inner[0].span, 0..0);
    }
}
