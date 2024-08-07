// This is free and unencumbered software released into the public domain.

//! This crate provides a basic KerML model.
//!
//! ```edition2021
//! # use kerml::*;
//! ```

#![no_std]

mod element;
pub use element::*;

mod namespace;
pub use namespace::*;

#[doc(hidden)]
pub mod prelude;
