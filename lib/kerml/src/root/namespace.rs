// This is free and unencumbered software released into the public domain.

use super::Element;
use crate::prelude::{String, Vec};

pub trait Namespace: Element {
    fn names_of(&self, _element: &dyn Element) -> Vec<String> {
        Vec::new()
    }
}
