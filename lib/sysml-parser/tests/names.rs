// This is free and unencumbered software released into the public domain.

use sysml_model::*;
use sysml_parser::{grammar::package, ParseResult};

#[test]
fn parse_short_name_only() -> ParseResult<'static, ()> {
    let input = r#"package <MyPkg>;"#;
    let (_, package) = package(input.into())?;
    assert_eq!(package.short_name(), Some("MyPkg"));
    assert_eq!(package.name(), None);
    Ok(())
}

#[test]
fn parse_name_only() -> ParseResult<'static, ()> {
    let input = r#"package MyPackage;"#;
    let (_, package) = package(input.into())?;
    assert_eq!(package.short_name(), None);
    assert_eq!(package.name(), Some("MyPackage"));
    Ok(())
}

#[test]
fn parse_both_names() -> ParseResult<'static, ()> {
    let input = r#"package <MyPkg> MyPackage;"#;
    let (_, package) = package(input.into())?;
    assert_eq!(package.short_name(), Some("MyPkg"));
    assert_eq!(package.name(), Some("MyPackage"));
    Ok(())
}

#[test]
fn parse_both_names_quoted() -> ParseResult<'static, ()> {
    let input = r#"package <'My Pkg'> 'My Package';"#;
    let (_, package) = package(input.into())?;
    assert_eq!(package.short_name(), Some("My Pkg"));
    assert_eq!(package.name(), Some("My Package"));
    Ok(())
}
