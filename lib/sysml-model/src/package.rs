// This is free and unencumbered software released into the public domain.

use crate::{Block, prelude::{String, ToString, Vec}};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Package {
    pub name: String,
    pub blocks: Vec<Block>,
}

impl Package {
    pub fn new(name: impl ToString) -> Self {
        Self { name: name.to_string(), blocks: Vec::new() }
    }

    pub fn with_blocks(name: impl ToString, blocks: Vec<Block>) -> Self {
        Self { name: name.to_string(), blocks }
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
