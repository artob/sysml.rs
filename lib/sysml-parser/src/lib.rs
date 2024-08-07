// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML parser.
//!
//! ```edition2021
//! # use sysml_parser::*;
//! ```

#![no_std]

mod block;
pub use block::*;

mod error;
pub use error::*;

pub mod grammar;

mod import;
pub use import::*;

mod package;
pub use package::*;

pub mod parser;
pub use parser::parse_string;

#[allow(unused)]
#[doc(hidden)]
use sysml_model::prelude;
