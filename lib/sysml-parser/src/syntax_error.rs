// This is free and unencumbered software released into the public domain.

use crate::lexer::Span;
use crate::prelude::{fmt, vec, Vec};

#[cfg(feature = "std")]
extern crate std;

pub type SyntaxResult<'a, T, E = SyntaxError<'a>> = Result<T, Err<'a, E>>;

pub type Err<'a, E = SyntaxError<'a>> = nom::Err<E>;

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxErrorKind {
    Context(&'static str),
    Char(char),
    Nom(nom::error::ErrorKind),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SyntaxError<'a> {
    pub errors: Vec<(SyntaxErrorKind, Span<'a>)>,
}

impl<'a> SyntaxError<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> fmt::Display for SyntaxError<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("parse error") // TODO
    }
}

// #[cfg(feature = "std")]
// impl From<std::io::Error> for ParseError<'_> {
//     fn from(error: std::io::Error) -> Self {
//         //nom::error::FromExternalError<Span<'a>>::from_ext error
//         todo!()
//     }
// }

impl<'a> Into<nom::Err<SyntaxError<'a>>> for SyntaxError<'a> {
    fn into(self) -> nom::Err<SyntaxError<'a>> {
        nom::Err::Failure(self)
    }
}

impl<'a> nom::error::ContextError<Span<'a>> for SyntaxError<'a> {
    fn add_context(input: Span<'a>, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push((SyntaxErrorKind::Context(ctx), input));
        other
    }
}

impl<'a, E> nom::error::FromExternalError<Span<'a>, E> for SyntaxError<'a> {
    fn from_external_error(input: Span<'a>, kind: nom::error::ErrorKind, _e: E) -> Self {
        use nom::error::ParseError as _;
        Self::from_error_kind(input, kind)
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for SyntaxError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self {
            errors: vec![(SyntaxErrorKind::Nom(kind), input)],
        }
    }

    fn append(input: Span<'a>, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.errors.push((SyntaxErrorKind::Nom(kind), input));
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self {
            errors: vec![(SyntaxErrorKind::Char(c), input)],
        }
    }
}
