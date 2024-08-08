// This is free and unencumbered software released into the public domain.

use super::Keyword;
use crate::{prelude::String, Error};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{map_res, recognize},
    error::ErrorKind,
    multi::many0, sequence::{delimited, pair},
    IResult
};
use sysml_model::QualifiedName;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Token {
    Name(String),
    QualifiedName(QualifiedName),
    Keyword(Keyword),
}

pub fn name(input: &str) -> IResult<&str, Token> {
    alt((basic_name, unrestricted_name))(input)
}

pub fn basic_name(input: &str) -> IResult<&str, Token> {
    let (input, name) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)?;

    Ok((input, Token::Name(String::from(name))))
}

pub fn unrestricted_name(input: &str) -> IResult<&str, Token> {
    let (input, name) = delimited(char('\''), is_not("'"), char('\''))(input)?;

    Ok((input, Token::Name(String::from(name))))
}

pub fn reserved_keyword(input: &str) -> IResult<&str, Keyword> {
    map_res(
        take_while1(|c: char| c.is_ascii_lowercase()),
        |s: &str| Keyword::try_from(s).or_else(|_| Err(Error::new(input, ErrorKind::Tag)))
    )(input)
}

#[cfg(test)]
mod tests {}
