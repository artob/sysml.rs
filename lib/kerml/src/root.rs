// This is free and unencumbered software released into the public domain.

//! The Root layer for KerML.
//!
//! ```edition2021
//! # use kerml::root::*;
//! ```

#![allow(unused_imports)]

mod annotation;
pub use annotation::*;

mod dependency;
pub use dependency::*;

mod element;
pub use element::*;

mod namespace;
pub use namespace::*;

mod qualified_name;
pub use qualified_name::*;

mod relationship;
pub use relationship::*;
