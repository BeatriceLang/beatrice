use std::hash::{Hash, Hasher};

use crate::span::{Span, Spanned};

#[derive(Clone, Debug, Eq)]
pub struct Ident(Spanned<String>);

impl Ident {
    pub const fn new(name: String, span: Span) -> Self {
        Self(Spanned::new(name, span))
    }

    pub fn as_str(&self) -> &str {
        &self.0.inner
    }

    pub fn span(&self) -> Span {
        self.0.span.clone()
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
