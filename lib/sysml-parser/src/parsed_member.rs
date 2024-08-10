// This is free and unencumbered software released into the public domain.

use crate::{ParsedAttribute, ParsedBlock, ParsedImport, ParsedPackage, ParsedPort};
use sysml_model::{Element, Namespace};

#[doc(hidden)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParsedMember {
    Import(ParsedImport),
    Package(ParsedPackage),
    BlockUsage(ParsedBlock),
    AttributeUsage(ParsedAttribute),
    PortUsage(ParsedPort),
}

impl Namespace for ParsedMember {}

impl Element for ParsedMember {
    fn name(&self) -> Option<&str> {
        use ParsedMember::*;
        match self {
            Import(import) => import.name(),
            Package(package) => package.name(),
            BlockUsage(usage) => usage.name(),
            AttributeUsage(usage) => usage.name(),
            PortUsage(usage) => usage.name(),
        }
    }

    fn short_name(&self) -> Option<&str> {
        use ParsedMember::*;
        match self {
            Import(import) => import.short_name(),
            Package(package) => package.short_name(),
            BlockUsage(usage) => usage.short_name(),
            AttributeUsage(usage) => usage.short_name(),
            PortUsage(usage) => usage.short_name(),
        }
    }
}
