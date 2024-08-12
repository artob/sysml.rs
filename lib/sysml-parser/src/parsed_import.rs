// This is free and unencumbered software released into the public domain.

use sysml_model::{Element, Import, QualifiedName, Relationship, VisibilityKind};

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParsedImport {
    pub imported_name: QualifiedName,
    pub import_all: bool,
    pub visibility: Option<VisibilityKind>,
}

impl ParsedImport {
    pub fn new(qualified_name: impl Into<QualifiedName>) -> Self {
        Self {
            imported_name: qualified_name.into(),
            import_all: false,
            visibility: None,
        }
    }

    pub fn is_wildcard(&self) -> bool {
        self.imported_name.last().map_or(false, |name| name == "*")
    }
}

impl From<&str> for ParsedImport {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl Import for ParsedImport {
    fn visibility(&self) -> VisibilityKind {
        self.visibility.unwrap_or(VisibilityKind::Public)
    }

    fn is_import_all(&self) -> bool {
        self.import_all
    }

    fn is_recursive(&self) -> bool {
        self.imported_name.last().map_or(false, |name| name == "**")
    }
}

impl Relationship for ParsedImport {}

impl Element for ParsedImport {}
