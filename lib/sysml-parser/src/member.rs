// This is free and unencumbered software released into the public domain.

use crate::{ParsedBlock, ParsedImport};
use sysml_model::Element;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParsedMember {
    Import(ParsedImport),
    BlockUsage(ParsedBlock),
}

impl Element for ParsedMember {}
