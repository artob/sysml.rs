// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, String};
use crate::SyntaxError;

#[cfg(feature = "std")]
extern crate std;

pub type ParseResult<T, E = ParseError> = Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),
    Syntax(SyntaxError<'static>),
    Other(String),
}

impl<'a> fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(kind) => write!(fmt, "IO error: {:?}", kind),
            ParseError::Syntax(error) => write!(fmt, "Syntax error: {:?}", error),
            ParseError::Other(error) => write!(fmt, "Other error: {:?}", error),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::Io(error.kind())
    }
}

impl From<nom::Err<SyntaxError<'static>>> for ParseError {
    fn from(error: nom::Err<SyntaxError<'static>>) -> Self {
        match error {
            nom::Err::Error(e) | nom::Err::Failure(e) => ParseError::Syntax(e),
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}

impl From<SyntaxError<'static>> for ParseError {
    fn from(error: SyntaxError<'static>) -> Self {
        ParseError::Syntax(error)
    }
}
