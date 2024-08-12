// This is free and unencumbered software released into the public domain.

use crate::ParsedMember;
use sysml_model::{
    prelude::{String, Vec},
    BlockUsage, Element, Feature, ItemUsage, Namespace, OccurrenceUsage, PartUsage, QualifiedName,
    Type, Usage,
};

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedBlock {
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub definition: Option<QualifiedName>,
    pub members: Vec<ParsedMember>,
}

impl From<&str> for ParsedBlock {
    fn from(name: &str) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

impl BlockUsage for ParsedBlock {}
impl PartUsage for ParsedBlock {}
impl ItemUsage for ParsedBlock {}
impl OccurrenceUsage for ParsedBlock {}
impl Usage for ParsedBlock {}
impl Feature for ParsedBlock {}
impl Type for ParsedBlock {}
impl Namespace for ParsedBlock {}

impl Element for ParsedBlock {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}
