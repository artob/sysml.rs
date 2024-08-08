// This is free and unencumbered software released into the public domain.

use crate::ParsedMember;
use sysml_model::{
    prelude::{String, Vec},
    Element, Feature, Namespace, OccurrenceUsage, PortUsage, Type, Usage,
};

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedPort {
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub members: Vec<ParsedMember>,
}

impl PortUsage for ParsedPort {}
impl OccurrenceUsage for ParsedPort {}
impl Usage for ParsedPort {}
impl Feature for ParsedPort {}
impl Type for ParsedPort {}
impl Namespace for ParsedPort {}

impl Element for ParsedPort {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}
