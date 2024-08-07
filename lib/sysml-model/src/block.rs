// This is free and unencumbered software released into the public domain.

use crate::{Element, prelude::{String, ToString, Vec}};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Block {
    pub(crate) name: String,
    pub(crate) short_name: Option<String>,
}

impl Block {
    pub fn new(name: impl ToString) -> Self {
        Self { name: name.to_string(), short_name: None }
    }
}

impl Element for Block {
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
    fn create_block() {
        assert_eq!(Block::new("MyBlock").name, "MyBlock");
    }
}
