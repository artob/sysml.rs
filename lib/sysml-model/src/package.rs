// This is free and unencumbered software released into the public domain.

use crate::{Block, Element, prelude::{String, ToString, Vec}};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Package {
    pub(crate) name: String,
    pub(crate) short_name: Option<String>,
    pub(crate) blocks: Vec<Block>,
}

impl Package {
    pub fn new(name: impl ToString) -> Self {
        Self::with_blocks(name, Vec::new())
    }

    pub fn with_blocks(name: impl ToString, blocks: Vec<Block>) -> Self {
        Self { name: name.to_string(), short_name: None, blocks }
    }

    pub fn blocks(&self) -> &Vec<Block> {
            &self.blocks
        }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Element for Package {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::prelude::*;
    use super::*;

    #[test]
    fn create_package() {
        assert_eq!(Package::new("MyPackage").name, "MyPackage");
    }
}
