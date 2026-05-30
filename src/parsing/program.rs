use chumsky::{IterParser, Parser, prelude::end};

use crate::{ast::Program, parsing::function::function, parsing_rule};

parsing_rule! {
    program -> Program{
        function()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| Program { functions })
    }
}
