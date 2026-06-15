use crate::{
    ast::{Ident, Item},
    check::Checker,
    diagnostic::{Diagnostic, DiagnosticKind},
};

impl<'a> Checker<'a> {
    pub fn check_duplicate_function(&mut self) {
        let mut checked = vec![];

        for item in &self.program.items {
            let Item::Function(function) = item;
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ast::{Block, Function, Ident, Item, Program, Type},
        check::Checker,
        diagnostic::{DiagnosticKind, Diagnostics},
    };

    fn ident(name: &str, span: std::ops::Range<usize>) -> Ident {
        Ident::new(name.into(), span)
    }

    fn function(name: Ident) -> Function {
        Function {
            name,
            params: vec![],
            return_type: Type::I32,
            body: Block { statements: vec![] },
        }
    }

    fn check_duplicate_function(program: &Program) -> Diagnostics {
        let mut diagnostics = Diagnostics::new("".into(), PathBuf::from("main.bea"));
        let mut checker = Checker {
            diagnostics: &mut diagnostics,
            program,
        };

        checker.check_duplicate_function();

        diagnostics
    }

    #[test]
    fn duplicate_function_reports_second_function_name_span() {
        let program = Program {
            items: vec![
                Item::Function(function(ident("main", 3..7))),
                Item::Function(function(ident("main", 20..24))),
            ],
        };

        let diagnostics = check_duplicate_function(&program);
        let diagnostics = diagnostics.iter().collect::<Vec<_>>();

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].span, 20..24);
        assert_eq!(diagnostics[0].kind, DiagnosticKind::Error);
        assert_eq!(diagnostics[0].message, "Function `main` is already defined");
        assert_eq!(diagnostics[0].label, diagnostics[0].message);
    }

    #[test]
    fn unique_functions_do_not_report_diagnostics() {
        let program = Program {
            items: vec![
                Item::Function(function(ident("main", 3..7))),
                Item::Function(function(ident("add", 20..23))),
            ],
        };

        let diagnostics = check_duplicate_function(&program);

        assert_eq!(diagnostics.iter().count(), 0);
    }
}
