// This is free and unencumbered software released into the public domain.

use crate::Relationship;

pub trait Import: Relationship {
    fn is_recursive(&self) -> bool {
        false
    }

    fn is_import_all(&self) -> bool {
        false
    }
}
