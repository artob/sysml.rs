// This is free and unencumbered software released into the public domain.

use crate::{OccurrenceDefinition, OccurrenceUsage, Structure};

pub trait ItemDefinition: OccurrenceDefinition + Structure {}

pub trait ItemUsage: OccurrenceUsage {}
