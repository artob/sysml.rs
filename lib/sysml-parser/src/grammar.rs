// This is free and unencumbered software released into the public domain.

use crate::{prelude::{String, Vec}, ParsedBlock, ParsedImport, ParsedMember, ParsedPackage};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult
};
use sysml_model::QualifiedName;

pub fn package(input: &str) -> IResult<&str, ParsedPackage> {
    let (input, (name, short_name, members)) = element(input, "package")?;

    Ok((input, ParsedPackage {
        name: name.map(String::from),
        short_name: short_name.map(String::from),
        members,
    }))
}

pub fn block(input: &str) -> IResult<&str, ParsedMember> {
    let (input, (name, short_name, members)) = element(input, "block")?;

    Ok((input, ParsedMember::Block(ParsedBlock {
        name: name.map(String::from),
        short_name: short_name.map(String::from),
        members,
    })))
}

pub fn element(input: &str, tag_name: impl AsRef<str>) -> IResult<&str, (Option<&str>, Option<&str>, Vec<ParsedMember>)> {
    let (input, _) = tag(tag_name.as_ref())(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (name, short_name)) = identification(input)?;
    let (input, _) = multispace0(input)?;
    let (input, members) = alt((
        map(char(';'), |_| Vec::new()),
        delimited(
            char('{'),
            preceded(multispace0, members),
            preceded(multispace0, char('}'))),
    ))(input)?;

    Ok((input, (name, short_name, members)))
}

fn members(input: &str) -> IResult<&str, Vec<ParsedMember>> {
    let (input, members) = many0(
        alt((
            terminated(import, multispace0),
            terminated(block, multispace0)
        ))
    )(input)?;

    Ok((input, members))
}

fn import(input: &str) -> IResult<&str, ParsedMember> {
    let (input, _) = tag("import")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = map(take_while(|c| c != ';'), QualifiedName::from)(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, ParsedMember::Import(ParsedImport::new(name))))
}

pub fn identification(input: &str) -> IResult<&str, (Option<&str>, Option<&str>)> {
    let (input, short_name) = opt(delimited(char('<'), name, char('>')))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = opt(name)(input)?;

    Ok((input, (name, short_name)))
}

pub fn name(input: &str) -> IResult<&str, &str> {
    alt((basic_name, unrestricted_name))(input)
}

pub fn basic_name(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

pub fn unrestricted_name(input: &str) -> IResult<&str, &str> {
    delimited(char('\''), is_not("'"), char('\''))(input)
}

#[cfg(test)]
mod tests {}
