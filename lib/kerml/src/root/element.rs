// This is free and unencumbered software released into the public domain.

use super::{Namespace, QualifiedName, Relationship};
use crate::prelude::{fmt, vec, String, Vec};

pub trait Element {
    /// The unique element ID, if any.
    fn id(&self) -> Option<&str> {
        None
    }

    /// Various alternative identifiers for this [`Element`].
    fn alias_ids(&self) -> Vec<&str> {
        Vec::new()
    }

    /// The name of the element.
    fn name(&self) -> Option<&str> {
        None
    }

    /// The short name of the element, if any.
    fn short_name(&self) -> Option<&str> {
        None
    }

    /// Whether this [`Element`] is contained in the ownership tree of
    /// a library model.
    fn is_library_element(&self) -> bool {
        false
    }

    /// The owner of this [`Element`], if any.
    fn owner(&self) -> Option<&dyn Element> {
        None
    }

    /// The [`Namespace`] that owns this [`Element`], if any.
    fn owning_namespace(&self) -> Option<&dyn Namespace> {
        None
    }

    /// The [`Relationship`] for which this [`Element`] is an
    /// `owned_related_element`, if any.
    fn owning_relationship(&self) -> Option<&dyn Relationship> {
        None
    }

    fn qualified_name(&self) -> Option<QualifiedName> {
        let owning_namespace = self.owning_namespace()?;
        let escaped_name = String::from(self.name()?);
        if owning_namespace.owner().is_none() {
            Some(QualifiedName::from(escaped_name))
        } else {
            let mut names = owning_namespace.qualified_name()?.to_vec();
            names.push(escaped_name);
            Some(QualifiedName::from(names))
        }
    }
}
