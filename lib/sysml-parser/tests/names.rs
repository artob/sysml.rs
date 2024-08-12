// This is free and unencumbered software released into the public domain.

use sysml_model::Element;
use sysml_parser::{grammar::package, SyntaxResult};

#[test]
fn parse_short_name_only() -> SyntaxResult<'static, ()> {
    let (_, package) = package(r#"package <MyPkg>;"#)?;
    assert_eq!(package.short_name(), Some("MyPkg"));
    assert_eq!(package.name(), None);
    Ok(())
}

#[test]
fn parse_name_only() -> SyntaxResult<'static, ()> {
    let (_, package) = package(r#"package MyPackage;"#)?;
    assert_eq!(package.short_name(), None);
    assert_eq!(package.name(), Some("MyPackage"));
    Ok(())
}

#[test]
fn parse_both_names() -> SyntaxResult<'static, ()> {
    let (_, package) = package(r#"package <MyPkg> MyPackage;"#)?;
    assert_eq!(package.short_name(), Some("MyPkg"));
    assert_eq!(package.name(), Some("MyPackage"));
    Ok(())
}

#[test]
fn parse_both_names_quoted() -> SyntaxResult<'static, ()> {
    let (_, package) = package(r#"package <'My Pkg'> 'My Package';"#)?;
    assert_eq!(package.short_name(), Some("My Pkg"));
    assert_eq!(package.name(), Some("My Package"));
    Ok(())
}
