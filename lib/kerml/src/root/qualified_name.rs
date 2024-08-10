// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, String, ToString, Vec};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QualifiedName(Vec<String>);

impl QualifiedName {
    pub fn new(names: Vec<String>) -> Self {
        Self(names)
    }

    pub fn to_string(&self) -> String {
        self.0.join("::")
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.0.clone()
    }

    pub fn to_tuple2(&self) -> (Option<&str>, Option<&str>) {
        (self.first(), self.second())
    }

    pub fn to_tuple3(&self) -> (Option<&str>, Option<&str>, Option<&str>) {
        (self.first(), self.second(), self.third())
    }

    pub fn first(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    pub fn second(&self) -> Option<&str> {
        self.0.get(1).map(String::as_str)
    }

    pub fn third(&self) -> Option<&str> {
        self.0.get(2).map(String::as_str)
    }

    pub fn last(&self) -> Option<&str> {
        self.0.last().map(String::as_str)
    }
}

impl From<Vec<String>> for QualifiedName {
    fn from(names: Vec<String>) -> Self {
        Self::new(names)
    }
}

impl From<String> for QualifiedName {
    fn from(names: String) -> Self {
        Self::from(names.as_str())
    }
}

impl From<&str> for QualifiedName {
    fn from(name: &str) -> Self {
        Self::new(name.trim().split("::").map(String::from).collect())
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
