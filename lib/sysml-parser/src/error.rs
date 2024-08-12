// This is free and unencumbered software released into the public domain.

use crate::lexer::Span;
use crate::prelude::{fmt, vec, Vec};

pub type ParseResult<'a, T, E = ParseError<'a>> = Result<T, Err<'a, E>>;

pub type Err<'a, E = ParseError<'a>> = nom::Err<E>;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    Context(&'static str),
    Char(char),
    Nom(nom::error::ErrorKind),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParseError<'a> {
    pub errors: Vec<(ParseErrorKind, Span<'a>)>,
}

impl<'a> ParseError<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("parse error") // TODO
    }
}

impl<'a> nom::error::ContextError<Span<'a>> for ParseError<'a> {
    fn add_context(input: Span<'a>, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push((ParseErrorKind::Context(ctx), input));
        other
    }
}

impl<'a, E> nom::error::FromExternalError<Span<'a>, E> for ParseError<'a> {
    fn from_external_error(input: Span<'a>, kind: nom::error::ErrorKind, _e: E) -> Self {
        use nom::error::ParseError as _;
        Self::from_error_kind(input, kind)
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self {
            errors: vec![(ParseErrorKind::Nom(kind), input)],
        }
    }

    fn append(input: Span<'a>, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.errors.push((ParseErrorKind::Nom(kind), input));
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self {
            errors: vec![(ParseErrorKind::Char(c), input)],
        }
    }
}
