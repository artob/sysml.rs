// This is free and unencumbered software released into the public domain.

use crate::{grammar::package, ParsedPackage};
use nom::character::complete::multispace0;

pub fn parse_string(input: &str) -> Result<ParsedPackage, nom::Err<nom::error::Error<&str>>> {
    let (input, _) = multispace0(input)?;
    let (_, package) = package(input)?;
    Ok(package)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::prelude::*;
    use crate::{ParsedBlock, ParsedPackage};
    use super::*;

    #[test]
    fn parse_empty_block() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let input = r#"package MyPackage { block MyBlock { } }"#;
        assert_eq!(parse_string(input)?, ParsedPackage::with_blocks("MyPackage", vec![ParsedBlock::new("MyBlock")]));
        Ok(())
    }
}
