// This is free and unencumbered software released into the public domain.

use sysml_parser::{grammar::package, SyntaxResult};

#[test]
fn parse_block_usage() -> SyntaxResult<'static, ()> {
    let input = r#"package MyPackage {
        block MyBlock : MyBlockType;
    }"#;
    let (_, _package) = package(input.into())?;
    Ok(())
}
