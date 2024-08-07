// This is free and unencumbered software released into the public domain.

use crate::Feature;

pub trait Usage: Feature {
    fn is_reference(&self) -> bool {
        false
    }

    fn is_variation(&self) -> bool {
        false
    }
}
