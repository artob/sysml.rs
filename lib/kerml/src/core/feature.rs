// This is free and unencumbered software released into the public domain.

use super::Type;

pub trait Feature: Type {
    fn is_unique(&self) -> bool {
        true
    }

    fn is_ordered(&self) -> bool {
        false
    }

    fn is_composite(&self) -> bool {
        false
    }

    fn is_end(&self) -> bool {
        false
    }

    fn is_derived(&self) -> bool {
        false
    }

    fn is_readonly(&self) -> bool {
        false
    }

    fn is_portion(&self) -> bool {
        false
    }
}
