// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML model.
//!
//! ```edition2021
//! # use sysml_model::*;
//! ```

#![no_std]
#![allow(unused)]

pub use kerml::core::*;
pub use kerml::kernel::*;
pub use kerml::prelude;
pub use kerml::root::*;

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
