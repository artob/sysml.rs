// This is free and unencumbered software released into the public domain.

use crate::{prelude::{Rc, String, Vec}, ParsedBlock, ParsedImport, ParsedPackage};
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
    let (input, blocks) = delimited(
        char('{'),
        many0(preceded(multispace0, block)),
        preceded(multispace0, char('}'))
    )(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, ParsedPackage::with_blocks(name, blocks) ))
}

pub fn block(input: &str) -> IResult<&str, ParsedBlock> {
    let (input, _) = tag("block")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, imports) = delimited(
        char('{'),
        preceded(multispace0, block_content),
        preceded(multispace0, char('}'))
    )(input)?;

    Ok((input, ParsedBlock {
        name: String::from(name),
        short_name: None, // TODO
        imports,
    }))
}

fn block_content(input: &str) -> IResult<&str, Vec<ParsedImport>> {
    let (input, imports) = many0(terminated(import, multispace0))(input)?;

    Ok((input, imports))
}

fn import(input: &str) -> IResult<&str, ParsedImport> {
    let (input, _) = tag("import")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = map(take_while(|c| c != ';'), QualifiedName::from)(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, ParsedImport::new(name)))
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
