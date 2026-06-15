use crate::{
    check::Checker,
    diagnostic::{Diagnostic, DiagnosticKind},
};

impl<'a> Checker<'a> {
    pub fn check_duplicate_function(&mut self) {
        let mut checked = vec![];

        for function in &self.program.functions {
            let name = &function.name;

            if checked.contains(name) {
                self.push_diagnostic(name);
            } else {
                checked.push(name.clone());
            }
        }
    }

    fn push_diagnostic(&mut self, name: &String) {
        let message = format!("Function `{name}` is already defined ");
        self.diagnostics.push(Diagnostic {
            // TODO
            span: 0..0,
            kind: DiagnosticKind::Error,
            label: message.clone(),
            message,
        });
    }
}
