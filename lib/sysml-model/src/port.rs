// This is free and unencumbered software released into the public domain.

use crate::{OccurrenceDefinition, OccurrenceUsage, Structure};

pub trait PortDefinition: OccurrenceDefinition + Structure {}

pub trait PortUsage: OccurrenceUsage {}
