// This is free and unencumbered software released into the public domain.

use crate::ParsedMember;
use sysml_model::{
    prelude::{String, Vec},
    AttributeUsage, Element, Feature, Namespace, Type, Usage,
};

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedAttribute {
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub members: Vec<ParsedMember>,
}

impl AttributeUsage for ParsedAttribute {}
impl Usage for ParsedAttribute {}
impl Feature for ParsedAttribute {}
impl Type for ParsedAttribute {}
impl Namespace for ParsedAttribute {}

impl Element for ParsedAttribute {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}
