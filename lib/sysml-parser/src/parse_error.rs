// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, String, ToString, Vec};
use crate::{SyntaxError, SyntaxErrorKind};

#[cfg(feature = "std")]
extern crate std;

pub type ParseResult<T, E = ParseError> = Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),
    Syntax(Vec<(SyntaxErrorKind, String)>),
    Other(String),
}

impl fmt::Display for ParseError {
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

impl<'a> From<nom::Err<SyntaxError<'a>>> for ParseError {
    fn from(err: nom::Err<SyntaxError<'a>>) -> Self {
        match err {
            nom::Err::Error(error) | nom::Err::Failure(error) => ParseError::from(error),
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}

impl<'a> From<SyntaxError<'a>> for ParseError {
    fn from(error: SyntaxError<'a>) -> Self {
        ParseError::Syntax(
            error
                .errors
                .iter()
                .map(|(kind, span)| (kind.clone(), span.to_string()))
                .collect(),
        )
    }
}
