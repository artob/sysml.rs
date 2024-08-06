// This is free and unencumbered software released into the public domain.

use crate::prelude::{String, ToString, Vec};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Block {
    pub name: String,
}

impl Block {
    pub fn new(name: impl ToString) -> Self {
        Self { name: name.to_string() }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::prelude::*;
    use super::*;

    #[test]
    fn create_block() {
        assert_eq!(Block::new("MyBlock").name, "MyBlock");
    }
}
