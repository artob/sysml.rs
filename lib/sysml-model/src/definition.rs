// This is free and unencumbered software released into the public domain.

pub trait Definition {}

pub enum DefinitionElement {
    Package,
    ItemDefinition,
    PartDefinition,
    PortDefinition,
}
