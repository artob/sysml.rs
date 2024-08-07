// This is free and unencumbered software released into the public domain.

use crate::{prelude::{Rc, String, Vec}, ParsedBlock, ParsedImport, ParsedMember, ParsedPackage};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult
};
use sysml_model::{Package, QualifiedName};

pub fn package(input: &str) -> IResult<&str, Rc<dyn Package>> {
    let (input, _) = tag("package")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, members) = delimited(
        char('{'),
        preceded(multispace0, members),
        preceded(multispace0, char('}'))
    )(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Rc::new(ParsedPackage {
        name: String::from(name),
        short_name: None, // TODO
        members,
    })))
}

pub fn block(input: &str) -> IResult<&str, ParsedMember> {
    let (input, _) = tag("block")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, members) = delimited(
        char('{'),
        preceded(multispace0, members),
        preceded(multispace0, char('}'))
    )(input)?;

    Ok((input, ParsedMember::Block(ParsedBlock {
        name: String::from(name),
        short_name: None, // TODO
        members,
    })))
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

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

#[cfg(test)]
mod tests {}
