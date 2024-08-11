// This is free and unencumbered software released into the public domain.

pub type ParseResult<'a, T, E = ParseError<'a>> = Result<T, Err<'a, E>>;

pub type Err<'a, E = ParseError<'a>> = nom::Err<E>;

pub type ParseError<'a> = nom::error::VerboseError<&'a str>;

//pub type IResult<'a, I, O, E = ParseError<'a>> = Result<(I, O), Err<'a, E>>;

#[doc(hidden)]
pub use nom::error::convert_error;
