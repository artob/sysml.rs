// This is free and unencumbered software released into the public domain.

use super::lexer::*;
use crate::{
    prelude::{String, Vec},
    ParsedAttribute, ParsedBlock, ParsedImport, ParsedMember, ParsedPackage, ParsedPort,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    error::context,
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use sysml_model::QualifiedName;

pub fn members(input: &str) -> IResult<&str, Vec<ParsedMember>> {
    let (input, members) = context(
        "tokens",
        many0(alt((
            terminated(map(import, ParsedMember::Import), multispace0),
            terminated(map(package, ParsedMember::Package), multispace0),
            terminated(map(block_usage, ParsedMember::BlockUsage), multispace0),
            terminated(
                map(attribute_usage, ParsedMember::AttributeUsage),
                multispace0,
            ),
            terminated(map(port_usage, ParsedMember::PortUsage), multispace0),
        ))),
    )(input)?;

    Ok((input, members))
}

pub fn import(input: &str) -> IResult<&str, ParsedImport> {
    let (input, _) = tag("import")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = map(take_while(|c| c != ';'), QualifiedName::from)(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, ParsedImport::new(name)))
}

pub fn package(input: &str) -> IResult<&str, ParsedPackage> {
    let (input, (name, short_name, _, members)) = element(input, "package")?;

    Ok((
        input,
        ParsedPackage {
            name: name.map(String::from),
            short_name: short_name.map(String::from),
            members,
        },
    ))
}

pub fn block_usage(input: &str) -> IResult<&str, ParsedBlock> {
    let (input, (name, short_name, definition, members)) = element(input, "block")?;

    Ok((
        input,
        ParsedBlock {
            name,
            short_name,
            definition,
            members,
        },
    ))
}

pub fn attribute_usage(input: &str) -> IResult<&str, ParsedAttribute> {
    let (input, (name, short_name, definition, members)) = element(input, "attribute")?;

    Ok((
        input,
        ParsedAttribute {
            name,
            short_name,
            definition,
            members,
        },
    ))
}

pub fn port_usage(input: &str) -> IResult<&str, ParsedPort> {
    let (input, (name, short_name, definition, members)) = element(input, "port")?;

    Ok((
        input,
        ParsedPort {
            name,
            short_name,
            definition,
            members,
        },
    ))
}

pub fn element<'a>(
    input: &'a str,
    tag_name: &'static str,
) -> IResult<
    &'a str,
    (
        Option<String>,
        Option<String>,
        Option<QualifiedName>,
        Vec<ParsedMember>,
    ),
> {
    let (input, _) = context(tag_name, tag(tag_name))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (name, short_name)) = identification(input)?;
    let (input, _) = multispace0(input)?;
    let (input, definition) = opt(delimited(
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

    Ok((input, (name, short_name, definition, members)))
}

pub fn identification(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    let (input, short_name) = opt(delimited(char('<'), name, char('>')))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = opt(name)(input)?;

    Ok((input, (name, short_name)))
}

#[cfg(test)]
mod tests {}
