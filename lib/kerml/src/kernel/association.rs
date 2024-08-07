// This is free and unencumbered software released into the public domain.

use crate::{core::Classifier, root::Relationship};

pub trait Association: Classifier + Relationship {}
