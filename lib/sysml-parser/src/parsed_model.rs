// This is free and unencumbered software released into the public domain.

use crate::ParsedMember;
use sysml_model::{prelude::Vec, Element, Namespace, Package};

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedModel {
    pub members: Vec<ParsedMember>,
}

impl ParsedModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn members(&self) -> &Vec<ParsedMember> {
        &self.members
    }

    pub fn add_member(&mut self, member: ParsedMember) {
        self.members.push(member);
    }
}

impl Package for ParsedModel {}
impl Namespace for ParsedModel {}
impl Element for ParsedModel {}
