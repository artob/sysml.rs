// This is free and unencumbered software released into the public domain.

use crate::root::Namespace;

pub trait Type: Namespace {
    fn is_abstract(&self) -> bool {
        false
    }

    fn is_sufficient(&self) -> bool {
        false
    }

    fn is_conjugated(&self) -> bool {
        false
    }

    fn specializes(&self ,_supertype: &Self) -> bool {
        false
    }
}
