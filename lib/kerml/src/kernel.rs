// This is free and unencumbered software released into the public domain.

//! The Kernel layer for KerML.
//!
//! ```edition2021
//! # use kerml::kernel::*;
//! ```

#![allow(unused_imports)]

mod association;
pub use association::*;

mod class;
pub use class::*;

mod connector;
pub use connector::*;

mod datatype;
pub use datatype::*;

mod package;
pub use package::*;

mod structure;
pub use structure::*;
