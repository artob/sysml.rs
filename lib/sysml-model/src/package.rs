// This is free and unencumbered software released into the public domain.

use crate::{Block, prelude::Vec};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Package {
    pub blocks: Vec<Block>,
}

impl Package {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn with_blocks(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
}
