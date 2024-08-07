// This is free and unencumbered software released into the public domain.

use sysml_model::{Element, Import, QualifiedName, Relationship};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedImport {
    pub(crate) qualified_name: QualifiedName,
}

impl ParsedImport {
    pub fn new(qualified_name: QualifiedName) -> Self {
        Self { qualified_name }
    }
}

impl Import for ParsedImport {}
impl Relationship for ParsedImport {}
impl Element for ParsedImport {}
