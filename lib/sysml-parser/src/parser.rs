// This is free and unencumbered software released into the public domain.

use crate::prelude::ToOwned;
use crate::{grammar::model, ParseError, ParseResult, ParsedModel};

#[cfg(feature = "std")]
extern crate std;

//#[cfg(feature = "std")]
//pub fn parse_from_file(pathname: impl AsRef<std::path::Path>) -> ParseResult<ParsedModel> {
//    parse_from_string(&std::fs::read_to_string(pathname)?)
//}

//#[cfg(feature = "std")]
//pub fn parse_from_reader(reader: impl std::io::Read) -> ParseResult<ParsedModel> {
//    parse_from_string(&std::io::read_to_string(reader)?)
//}

pub fn parse_from_string(input: &str) -> ParseResult<ParsedModel> {
    let (_, model) = model(input).map_err(|err| match err {
        nom::Err::Error(error) | nom::Err::Failure(error) => ParseError::Syntax(error.to_owned()),
        nom::Err::Incomplete(_) => unreachable!(),
    })?;
    Ok(model)
}
