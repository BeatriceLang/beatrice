#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    I32,
    U32,
    String,
    Bool,
    Ptr(Box<Self>),
    Struct(String),
    Array { element_ty: Box<Self>, size: usize },
}

impl Type {
    pub(crate) const fn signed(&self) -> Option<bool> {
        match self {
            Self::U32 => Some(false),
            Self::I32 => Some(true),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn reports_integer_signedness() {
        assert_eq!(Type::I32.signed(), Some(true));
        assert_eq!(Type::U32.signed(), Some(false));
    }

    #[test]
    fn reports_no_signedness_for_non_integer_types() {
        assert_eq!(Type::String.signed(), None);
        assert_eq!(Type::Bool.signed(), None);
        assert_eq!(Type::Ptr(Box::new(Type::I32)).signed(), None);
        assert_eq!(Type::Struct("Point".into()).signed(), None);
    }
}
