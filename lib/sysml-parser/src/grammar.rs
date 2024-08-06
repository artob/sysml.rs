// This is free and unencumbered software released into the public domain.

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair},
    IResult
};
use sysml_model::{Block, Package};

pub fn package(input: &str) -> IResult<&str, Package> {
    let (input, _) = tag("package")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, blocks) = delimited(
        char('{'),
        many0(delimited(multispace0, block, multispace0)),
        char('}')
    )(input)?;

    Ok((input, Package::with_blocks(name, blocks) ))
}

pub fn block(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag("block")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = delimited(
        char('{'),
        multispace0, // TODO
        char('}')
    )(input)?;

    Ok((input, Block::new(name)))
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
mod tests {
    extern crate std;
    //use crate::prelude::*;
    //use super::*;
}
