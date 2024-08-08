// This is free and unencumbered software released into the public domain.

use crate::Classifier;

pub trait Definition: Classifier {
    fn is_variation(&self) -> bool {
        false
    }
}

enum DefinitionElement {
    Package,
    ItemDefinition,
    PartDefinition,
    PortDefinition,
    // TODO
}
