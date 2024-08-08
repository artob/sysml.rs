// This is free and unencumbered software released into the public domain.

use crate::{ConnectionDefinition, ConnectionUsage};

pub trait InterfaceDefinition: ConnectionDefinition {}

pub trait InterfaceUsage: ConnectionUsage {}
