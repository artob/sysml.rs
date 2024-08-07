// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML model.
//!
//! ```edition2021
//! # use sysml_model::*;
//! ```

#![no_std]
#![allow(unused)]

pub use kerml::*;

mod block;
pub use block::*;

mod element;
pub use element::*;

mod package;
pub use package::*;
