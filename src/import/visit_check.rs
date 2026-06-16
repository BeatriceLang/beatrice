use std::path::PathBuf;

use crate::{
    diagnostic::{Diagnostic, DiagnosticKind, Diagnostics},
    import::ImportProcessor,
};

impl<'a> ImportProcessor<'a> {
    #[must_use]
    pub(super) fn visit_check(&mut self, path: PathBuf) -> bool {
        if self.visited.contains(&path) {
            true
        } else {
            self.visited.push(path);
            false
        }
    }
}
