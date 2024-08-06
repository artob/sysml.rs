// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML model.
//!
//! ```edition2021
//! # use sysml_model::*;
//! ```

#![no_std]
#![allow(unused)]

mod block;
pub use block::*;

mod package;
pub use package::*;

#[doc(hidden)]
pub mod prelude;
