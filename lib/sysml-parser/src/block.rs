// This is free and unencumbered software released into the public domain.

use sysml_model::{prelude::{String, ToString}, Block, Element, Item, Part};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedBlock {
    pub(crate) name: String,
    pub(crate) short_name: Option<String>,
}

impl ParsedBlock {
    pub fn new(name: impl ToString) -> Self {
        Self { name: name.to_string(), short_name: None }
    }
}

impl Element for ParsedBlock {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}

impl Item for ParsedBlock {}
impl Part for ParsedBlock {}
impl Block for ParsedBlock {}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn create_block() {
        assert_eq!(ParsedBlock::new("MyBlock").name, "MyBlock");
    }
}
