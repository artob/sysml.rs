// This is free and unencumbered software released into the public domain.

use crate::root::Namespace;

pub trait Package: Namespace {}

pub trait LibraryPackage: Package {
    fn is_standard(&self) -> bool {
        false
    }
}
