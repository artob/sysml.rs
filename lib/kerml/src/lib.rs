// This is free and unencumbered software released into the public domain.

//! This crate provides a basic KerML model.
//!
//! ```edition2021
//! # use kerml::*;
//! ```

#![no_std]

pub mod core;
pub use core::*;

pub mod kernel;
pub use kernel::*;

#[doc(hidden)]
pub mod prelude;

pub mod root;
pub use root::*;
