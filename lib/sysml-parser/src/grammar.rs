// This is free and unencumbered software released into the public domain.

use crate::{prelude::Rc, ParsedBlock, ParsedPackage};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult
};
use sysml_model::Package;

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
    let (input, _) = delimited(
        char('{'),
        multispace0, // TODO
        char('}')
    )(input)?;

    Ok((input, ParsedBlock::new(name)))
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
