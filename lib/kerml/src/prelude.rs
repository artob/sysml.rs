// This is free and unencumbered software released into the public domain.

#![allow(unused)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as alloc;

pub use alloc::{
    vec,
    vec::Vec,
    string::{String, ToString},
};

pub use core::{
    result::Result,
};