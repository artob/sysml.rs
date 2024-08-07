// This is free and unencumbered software released into the public domain.

use crate::ParsedBlock;
use sysml_model::{prelude::{Rc, String, ToString, Vec}, Element, Namespace, Package};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedPackage {
    pub(crate) name: String,
    pub(crate) short_name: Option<String>,
    pub(crate) blocks: Vec<ParsedBlock>,
}

impl ParsedPackage {
    pub fn new(name: impl ToString) -> Rc<Self> {
        Self::with_blocks(name, Vec::new())
    }

    pub fn with_blocks(name: impl ToString, blocks: Vec<ParsedBlock>) -> Rc<Self> {
        Rc::new(Self { name: name.to_string(), short_name: None, blocks })
    }

    pub fn blocks(&self) -> &Vec<ParsedBlock> {
        &self.blocks
    }

    pub fn add_block(&mut self, block: ParsedBlock) {
        self.blocks.push(block);
    }
}

impl Element for ParsedPackage {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}

impl Namespace for ParsedPackage {}
impl Package for ParsedPackage {}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn create_package() {
        assert_eq!(ParsedPackage::new("MyPackage").name, "MyPackage");
    }
}
