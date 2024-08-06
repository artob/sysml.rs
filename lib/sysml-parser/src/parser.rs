// This is free and unencumbered software released into the public domain.

use crate::grammar::package;
use nom::character::complete::multispace0;
use sysml_model::Package;

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
    use sysml_model::{Block, Package};

    #[test]
    fn parse_empty_block() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let input = r#"package MyPackage { block MyBlock { } }"#;
        assert_eq!(parse_string(input)?, Package::with_blocks("MyPackage", vec![Block::new("MyBlock")]));
        Ok(())
    }
}
