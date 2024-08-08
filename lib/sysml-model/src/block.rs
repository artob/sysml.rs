// This is free and unencumbered software released into the public domain.

use crate::{PartDefinition, PartUsage};

pub trait BlockDefinition: PartDefinition {}

pub trait BlockUsage: PartUsage {}
