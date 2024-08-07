// This is free and unencumbered software released into the public domain.

use super::Element;
use crate::prelude::{fmt, String, Vec};

pub trait Namespace: Element {
    fn names_of(&self, _element: &dyn Element) -> Vec<String> {
        Vec::new()
    }
}

impl fmt::Debug for dyn Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for dyn Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.short_name(), self.name()) {
            (Some(short_name), Some(name)) => write!(f, "namespace <'{}'> '{}'", short_name, name),
            (Some(short_name), None) => write!(f, "namespace <'{}'>", short_name),
            (None, Some(name)) => write!(f, "namespace '{}'", name),
            (None, None) => write!(f, "namespace"),
        }
    }
}
