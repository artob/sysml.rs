// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML parser.
//!
//! ```edition2021
//! # use sysml_parser::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[allow(unused)]
#[doc(hidden)]
use sysml_model::prelude;

pub mod grammar;

pub mod keyword;
pub use keyword::*;

pub mod lexer;

mod parse_error;
pub use parse_error::*;

mod parsed_attribute;
pub use parsed_attribute::*;

mod parsed_block;
pub use parsed_block::*;

mod parsed_import;
pub use parsed_import::*;

mod parsed_member;
pub use parsed_member::*;

mod parsed_model;
pub use parsed_model::*;

mod parsed_package;
pub use parsed_package::*;

mod parsed_port;
pub use parsed_port::*;

pub mod parser;
pub use parser::*;

pub mod span;
pub use span::*;

pub mod spanned;
pub use spanned::*;

mod syntax_error;
pub use syntax_error::*;
