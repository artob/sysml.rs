// This is free and unencumbered software released into the public domain.

use sysml_model::{prelude::*, *};
use sysml_parser::{parse_from_string, ParseResult, ParsedBlock, ParsedMember, ParsedPackage};

#[test]
fn parse_package_empty() -> ParseResult<'static, ()> {
    let input = r#"package MyPackage {}"#;
    let reference: Rc<dyn Package> = ParsedPackage::new("MyPackage");
    assert_eq!(parse_from_string(input)?.to_string(), reference.to_string());
    Ok(())
}

#[test]
fn parse_package_imports() -> ParseResult<'static, ()> {
    let input = r#"package MyPackage {
        import Protolog::*;
    }"#;
    let _ = parse_from_string(input)?;
    Ok(())
}

#[test]
fn parse_block_empty() -> ParseResult<'static, ()> {
    let input = r#"package MyPackage { block MyBlock {} }"#;
    let reference: Rc<dyn Package> = ParsedPackage::with_members(
        "MyPackage",
        vec![ParsedMember::BlockUsage(ParsedBlock {
            name: Some("MyBlock".to_string()),
            ..Default::default()
        })],
    );
    assert_eq!(parse_from_string(input)?.to_string(), reference.to_string());
    Ok(())
}

#[test]
fn parse_block_imports() -> ParseResult<'static, ()> {
    let input = r#"package MyPackage {
        block MyBlock {
            import Protolog::*;
        }
    }"#;
    let _ = parse_from_string(input)?;
    Ok(())
}
