// This is free and unencumbered software released into the public domain.

use crate::{Class, Definition, Usage};

pub trait OccurrenceDefinition: Definition + Class {
    fn is_individual(&self) -> bool {
        false
    }
}

pub trait OccurrenceUsage: Usage {
    fn is_individual(&self) -> bool {
        false
    }

    fn portion_kind(&self) -> Option<PortionKind> {
        None
    }
}

pub enum PortionKind {
    TimeSlice,
    Snapshot,
}
