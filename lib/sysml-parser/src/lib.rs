// This is free and unencumbered software released into the public domain.

//! This crate provides a basic SysML parser.
//!
//! ```edition2021
//! # use sysml_parser::*;
//! ```

#![no_std]

#[allow(unused)]
#[doc(hidden)]
use sysml_model::prelude;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair},
    IResult
};
use sysml_model::*;

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

fn block(input: &str) -> IResult<&str, Block> {
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

fn package(input: &str) -> IResult<&str, Package> {
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

pub fn parse_string(input: &str) -> Result<Package, nom::Err<nom::error::Error<&str>>> {
    let (input, _) = multispace0(input)?;
    let (_, package) = package(input)?;
    Ok(package)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::prelude::*;
    use super::*;

    #[test]
    fn parse_empty_block() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let input = r#"package MyPackage { block MyBlock { } }"#;
        assert_eq!(parse_string(input)?, Package::with_blocks("MyPackage", vec![Block::new("MyBlock")]));
        Ok(())
    }
}
