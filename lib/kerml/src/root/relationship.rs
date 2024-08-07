// This is free and unencumbered software released into the public domain.

use super::Element;

pub trait Relationship: Element {
    fn is_implied(&self) -> bool {
        false
    }
}
