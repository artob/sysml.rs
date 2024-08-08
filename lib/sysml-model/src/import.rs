// This is free and unencumbered software released into the public domain.

use crate::Relationship;

pub trait Import: Relationship {
    fn visibility(&self) -> VisibilityKind {
        VisibilityKind::Public
    }

    fn is_recursive(&self) -> bool {
        false
    }

    fn is_import_all(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VisibilityKind {
    #[default]
    Public,
    Private,
    Protected,
}
