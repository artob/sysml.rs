// This is free and unencumbered software released into the public domain.

use sysml_parser::{
    parse_from_string, ParseResult, ParsedBlock, ParsedMember::*, ParsedModel, ParsedPackage,
};

#[test]
fn parse_package_empty() -> ParseResult<()> {
    assert_eq!(
        parse_from_string(r#"package MyPackage {}"#)?,
        ParsedModel {
            members: vec![Package(ParsedPackage::new("MyPackage"))],
        }
    );
    Ok(())
}

#[test]
fn parse_package_imports() -> ParseResult<()> {
    assert_eq!(
        parse_from_string(
            r#"
                package MyPackage {
                    import SysML::*;
                }
            "#
        )?,
        ParsedModel {
            members: vec![Package(ParsedPackage::with_members(
                "MyPackage",
                vec![Import("SysML::*".into())]
            ))],
        }
    );
    Ok(())
}

#[test]
fn parse_block_empty() -> ParseResult<()> {
    assert_eq!(
        parse_from_string(r#"package MyPackage { block MyBlock {} }"#)?,
        ParsedModel {
            members: vec![Package(ParsedPackage::with_members(
                "MyPackage",
                vec![BlockUsage(ParsedBlock {
                    name: Some("MyBlock".into()),
                    ..Default::default()
                })],
            ))],
        }
    );
    Ok(())
}

#[test]
fn parse_block_imports() -> ParseResult<()> {
    assert_eq!(
        parse_from_string(
            r#"
                package MyPackage {
                    block MyBlock {
                        import Protolog::*;
                    }
                }
            "#
        )?,
        ParsedModel {
            members: vec![Package(ParsedPackage::with_members(
                "MyPackage",
                vec![BlockUsage(ParsedBlock {
                    name: Some("MyBlock".into()),
                    members: vec![Import("Protolog::*".into())],
                    ..Default::default()
                })],
            ))],
        }
    );
    Ok(())
}
