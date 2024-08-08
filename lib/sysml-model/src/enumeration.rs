// This is free and unencumbered software released into the public domain.

use crate::{AttributeDefinition, AttributeUsage};

pub trait EnumerationDefinition: AttributeDefinition {
    fn is_variation(&self) -> bool {
        true
    }
}

pub trait EnumerationUsage: AttributeUsage {}
