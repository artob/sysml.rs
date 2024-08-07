// This is free and unencumbered software released into the public domain.

use crate::{prelude::fmt, root::Namespace};

pub trait Package: Namespace {}

pub trait LibraryPackage: Package {
    fn is_standard(&self) -> bool {
        false
    }
}

impl fmt::Debug for dyn Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for dyn Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.short_name(), self.name()) {
            (Some(short_name), Some(name)) => write!(f, "package <'{}'> '{}'", short_name, name),
            (Some(short_name), None) => write!(f, "package <'{}'>", short_name),
            (None, Some(name)) => write!(f, "package '{}'", name),
            (None, None) => write!(f, "package"),
        }
    }
}
