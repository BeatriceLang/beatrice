use crate::{
    ast::Ident,
    check::Checker,
    diagnostic::{Diagnostic, DiagnosticKind},
};

impl<'a> Checker<'a> {
    pub fn check_duplicate_function(&mut self) {
        let mut checked = vec![];

        for function in &self.program.functions {
            let name = &function.name;

            if checked.contains(&name.as_str()) {
                self.push_diagnostic(name);
            } else {
                checked.push(name.as_str());
            }
        }
    }

    fn push_diagnostic(&mut self, name: &Ident) {
        let message = format!("Function `{}` is already defined", name.as_str());
        self.diagnostics.push(Diagnostic {
            span: name.span(),
            kind: DiagnosticKind::Error,
            label: message.clone(),
            message,
        });
    }
}
