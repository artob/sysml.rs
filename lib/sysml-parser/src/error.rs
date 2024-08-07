// This is free and unencumbered software released into the public domain.

pub type ParseResult<'a, T, E = Error<'a>> = Result<T, Err<'a, E>>;

pub type Err<'a, E = Error<'a>> = nom::Err<E>;

pub type Error<'a> = nom::error::Error<&'a str>;
