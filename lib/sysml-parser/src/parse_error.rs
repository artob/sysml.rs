// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, String};
use crate::SyntaxError;

#[cfg(feature = "std")]
extern crate std;

pub type ParseResult<'a, T, E = ParseError<'a>> = Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError<'a> {
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),
    Syntax(SyntaxError<'a>),
    Other(String),
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(kind) => write!(fmt, "IO error: {:?}", kind),
            ParseError::Syntax(error) => write!(fmt, "Syntax error: {:?}", error),
            ParseError::Other(error) => write!(fmt, "Other error: {:?}", error),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ParseError<'_> {
    fn from(error: std::io::Error) -> Self {
        ParseError::Io(error.kind())
    }
}

impl<'a> From<nom::Err<SyntaxError<'a>>> for ParseError<'a> {
    fn from(error: nom::Err<SyntaxError<'a>>) -> Self {
        match error {
            nom::Err::Error(e) | nom::Err::Failure(e) => ParseError::Syntax(e),
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}

impl<'a> From<SyntaxError<'a>> for ParseError<'a> {
    fn from(error: SyntaxError<'a>) -> Self {
        ParseError::Syntax(error)
    }
}
