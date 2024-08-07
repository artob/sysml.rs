// This is free and unencumbered software released into the public domain.

use crate::{grammar::package, prelude::Rc};
use nom::character::complete::multispace0;
use sysml_model::Package;

pub fn parse_string(input: &str) -> Result<Rc<dyn Package>, nom::Err<nom::error::Error<&str>>> {
    let (input, _) = multispace0(input)?;
    let (_, package) = package(input)?;
    Ok(Rc::new(package))
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
        let reference: Box<dyn Package> = Box::new(ParsedPackage::with_blocks("MyPackage", vec![ParsedBlock::new("MyBlock")]));
        assert_eq!(parse_string(input)?.to_string(), reference.to_string());
        Ok(())
    }
}
