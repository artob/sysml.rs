// This is free and unencumbered software released into the public domain.

use crate::{ItemDefinition, ItemUsage};

pub trait PartDefinition: ItemDefinition {}

pub trait PartUsage: ItemUsage {}
