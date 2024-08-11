// This is free and unencumbered software released into the public domain.

use super::Keyword;
use crate::{
    prelude::{String, Vec},
    Error,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, multispace0},
    combinator::{map, map_res, recognize},
    error::{context, ErrorKind},
    multi::many0,
    sequence::{delimited, pair, terminated},
    IResult,
};
use sysml_model::QualifiedName;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Token {
    Keyword(Keyword),
    Name(String),
    QualifiedName(QualifiedName),
}

pub fn tokens(input: &str) -> IResult<&str, Vec<Token>> {
    context("tokens", many0(delimited(multispace0, token, multispace0)))(input)
}

pub fn token(input: &str) -> IResult<&str, Token> {
    context(
        "token",
        alt((
            map(reserved_keyword, Token::Keyword),
            map(name, Token::Name),
            map(qualified_name, Token::QualifiedName),
        )),
    )(input)
}

pub fn qualified_name(input: &str) -> IResult<&str, QualifiedName> {
    let (input, mut names) = many0(terminated(name, tag("::")))(input)?;
    let (input, name) = name(input)?;
    names.push(name);

    Ok((input, QualifiedName::new(names)))
}

pub fn name(input: &str) -> IResult<&str, String> {
    context("name", alt((basic_name, unrestricted_name)))(input)
}

pub fn basic_name(input: &str) -> IResult<&str, String> {
    let (input, name) = context(
        "basic_name",
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
    )(input)?;

    Ok((input, String::from(name)))
}

pub fn unrestricted_name(input: &str) -> IResult<&str, String> {
    let (input, name) = context(
        "unrestricted_name",
        delimited(char('\''), is_not("'"), char('\'')),
    )(input)?;

    Ok((input, String::from(name)))
}

pub fn reserved_keyword(input: &str) -> IResult<&str, Keyword> {
    context(
        "reserved_keyword",
        map_res(take_while1(|c: char| c.is_ascii_lowercase()), |s: &str| {
            Keyword::try_from(s).or_else(|_| Err(Error::new(input, ErrorKind::Tag)))
        }),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_block_keyword() {
        let input = r#"block"#;
        assert_eq!(
            token(input).map(|(_, token)| token),
            Ok(Token::Keyword(Keyword::Block))
        );
    }

    #[test]
    fn lex_basic_name() {
        let input = r#"MyPackage"#;
        assert_eq!(
            token(input).map(|(_, token)| token),
            Ok(Token::Name(String::from("MyPackage")))
        );
    }

    #[test]
    fn lex_quoted_name() {
        let input = r#"'My Package'"#;
        assert_eq!(
            token(input).map(|(_, token)| token),
            Ok(Token::Name(String::from("My Package")))
        );
    }
}
