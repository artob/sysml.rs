// This is free and unencumbered software released into the public domain.

use super::lexer::*;
use crate::{
    prelude::{String, Vec},
    ParsedBlock, ParsedImport, ParsedMember, ParsedPackage,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use sysml_model::QualifiedName;

pub fn package(input: &str) -> IResult<&str, ParsedPackage> {
    let (input, (name, short_name, members)) = element(input, "package")?;

    Ok((
        input,
        ParsedPackage {
            name: name.map(String::from),
            short_name: short_name.map(String::from),
            members,
        },
    ))
}

pub fn members(input: &str) -> IResult<&str, Vec<ParsedMember>> {
    let (input, members) = many0(alt((
        terminated(import, multispace0),
        terminated(block_usage, multispace0),
    )))(input)?;

    Ok((input, members))
}

pub fn import(input: &str) -> IResult<&str, ParsedMember> {
    let (input, _) = tag("import")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = map(take_while(|c| c != ';'), QualifiedName::from)(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, ParsedMember::Import(ParsedImport::new(name))))
}

pub fn block_usage(input: &str) -> IResult<&str, ParsedMember> {
    let (input, (name, short_name, members)) = element(input, "block")?;

    Ok((
        input,
        ParsedMember::BlockUsage(ParsedBlock {
            name,
            short_name,
            members,
        }),
    ))
}

pub fn element(
    input: &str,
    tag_name: impl AsRef<str>,
) -> IResult<&str, (Option<String>, Option<String>, Vec<ParsedMember>)> {
    let (input, _) = tag(tag_name.as_ref())(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (name, short_name)) = identification(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(delimited(
        terminated(char(':'), multispace0),
        qualified_name,
        multispace0,
    ))(input)?;
    let (input, members) = alt((
        map(char(';'), |_| Vec::new()),
        delimited(
            char('{'),
            preceded(multispace0, members),
            preceded(multispace0, char('}')),
        ),
    ))(input)?;

    Ok((input, (name, short_name, members)))
}

pub fn identification(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    let (input, short_name) = opt(delimited(char('<'), name, char('>')))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = opt(name)(input)?;

    Ok((input, (name, short_name)))
}

#[cfg(test)]
mod tests {}
