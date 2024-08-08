// This is free and unencumbered software released into the public domain.

use crate::ParsedMember;
use sysml_model::{
    prelude::{String, ToString, Vec},
    Block, Element, Item, Part,
};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedBlock {
    pub(crate) name: Option<String>,
    pub(crate) short_name: Option<String>,
    pub(crate) members: Vec<ParsedMember>,
}

impl ParsedBlock {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: Some(name.to_string()),
            ..Default::default()
        }
    }
}

impl Block for ParsedBlock {}
impl Part for ParsedBlock {}
impl Item for ParsedBlock {}

impl Element for ParsedBlock {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn create_block() {
        assert_eq!(
            ParsedBlock::new("MyBlock").name,
            Some("MyBlock".to_string())
        );
    }
}
