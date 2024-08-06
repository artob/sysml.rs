// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Block {}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }
}
