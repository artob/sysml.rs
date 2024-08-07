// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML model.
//!
//! ```edition2021
//! # use sysml_model::*;
//! ```

#![no_std]
#![allow(unused)]

pub use kerml::prelude;
pub use kerml::core::{Classifier, Feature, Type};
pub use kerml::kernel::{Association, Class, Connector, DataType, Package, Structure};
pub use kerml::root::{Annotation, Dependency, Element, Namespace, Relationship};

mod attribute;
pub use attribute::*;

mod block;
pub use block::*;

mod connection;
pub use connection::*;

mod definition;
pub use definition::*;

mod enumeration;
pub use enumeration::*;

mod import;
pub use import::*;

mod interface;
pub use interface::*;

mod item;
pub use item::*;

mod occurrence;
pub use occurrence::*;

mod part;
pub use part::*;

mod port;
pub use port::*;

mod usage;
pub use usage::*;
