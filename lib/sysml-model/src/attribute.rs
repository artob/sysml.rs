// This is free and unencumbered software released into the public domain.

use crate::{DataType, Definition, Usage};

pub trait AttributeDefinition: Definition + DataType {}

pub trait AttributeUsage: Usage {
    fn is_reference(&self) -> bool {
        true
    }
}
