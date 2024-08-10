// This is free and unencumbered software released into the public domain.

use crate::ParsedMember;
use sysml_model::{
    prelude::{Rc, String, ToString, Vec},
    Element, Namespace, Package,
};

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedPackage {
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub members: Vec<ParsedMember>,
}

impl ParsedPackage {
    pub fn new(name: impl ToString) -> Rc<Self> {
        Self::with_members(name, Vec::new())
    }

    pub fn with_members(name: impl ToString, members: Vec<ParsedMember>) -> Rc<Self> {
        Rc::new(Self {
            name: Some(name.to_string()),
            short_name: None,
            members,
        })
    }

    pub fn members(&self) -> &Vec<ParsedMember> {
        &self.members
    }

    pub fn add_member(&mut self, member: ParsedMember) {
        self.members.push(member);
    }
}

impl Package for ParsedPackage {}
impl Namespace for ParsedPackage {}

impl Element for ParsedPackage {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}