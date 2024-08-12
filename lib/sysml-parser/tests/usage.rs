// This is free and unencumbered software released into the public domain.

use sysml_parser::{grammar::package, ParsedBlock, ParsedMember::*, ParsedPackage, SyntaxResult};

#[test]
fn parse_block_usage() -> SyntaxResult<'static, ()> {
    assert_eq!(
        package(
            r#"
                package MyPackage {
                    block MyBlock : MyBlockType;
                }
            "#
            .trim(),
        )?
        .1,
        ParsedPackage::with_members(
            "MyPackage",
            vec![BlockUsage(ParsedBlock {
                name: Some("MyBlock".into()),
                definition: Some("MyBlockType".into()),
                ..Default::default()
            })],
        )
    );
    Ok(())
}
