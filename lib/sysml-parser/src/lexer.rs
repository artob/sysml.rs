// This is free and unencumbered software released into the public domain.

use super::Keyword;
use crate::{
    prelude::{String, Vec},
    Span, SyntaxError, SyntaxResult,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, multispace0},
    combinator::{map, map_res, recognize},
    error::{context, ErrorKind},
    multi::many0,
    sequence::{delimited, pair, terminated},
};
use sysml_model::QualifiedName;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Token {
    Keyword(Keyword),
    Name(String),
    QualifiedName(QualifiedName),
}

pub fn tokens<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, Vec<Token>)> {
    context("tokens", many0(delimited(multispace0, token, multispace0)))(input.into())
}

pub fn token<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, Token)> {
    context(
        "token",
        alt((
            map(reserved_keyword, Token::Keyword),
            map(name, Token::Name),
            map(qualified_name, Token::QualifiedName),
        )),
    )(input.into())
}

pub fn qualified_name<'a>(
    input: impl Into<Span<'a>>,
) -> SyntaxResult<'a, (Span<'a>, QualifiedName)> {
    let (input, mut names) = many0(terminated(name, tag("::")))(input.into())?;
    let (input, name) = name(input)?;
    names.push(name);

    Ok((input, QualifiedName::new(names)))
}

pub fn name<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, String)> {
    context("name", alt((basic_name, unrestricted_name)))(input.into())
}

pub fn basic_name<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, String)> {
    let (input, name) = context(
        "basic_name",
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
    )(input.into())?;

    Ok((input, String::from(name.into_fragment())))
}

pub fn unrestricted_name<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, String)> {
    let (input, name) = context(
        "unrestricted_name",
        delimited(char('\''), is_not("'"), char('\'')),
    )(input.into())?;

    Ok((input, String::from(name.into_fragment())))
}

pub fn reserved_keyword<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, Keyword)> {
    use nom::error::ParseError as _; // for from_error_kind
    use nom::AsChar;
    context(
        "reserved_keyword",
        map_res(
            take_while1(|c| AsChar::as_char(c).is_ascii_lowercase()),
            |span: Span| {
                Keyword::try_from(span)
                    .or_else(|input| Err(SyntaxError::from_error_kind(input, ErrorKind::Tag)))
            },
        ),
    )(input.into())
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
