// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML support.
//!
//! ```edition2021
//! # use sysml::model::*;
//! # use sysml::parser::*;
//! ```

#![no_std]
#![allow(unused)]

#[doc(hidden)]
pub mod prelude;

mod feature;
pub use feature::*;

pub mod model {
    pub use sysml_model::*;
}

pub mod parser {
    pub use sysml_parser::*;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
