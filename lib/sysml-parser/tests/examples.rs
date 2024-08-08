// This is free and unencumbered software released into the public domain.

use sysml_parser::{parse_string, ParseResult};

#[test]
fn parse_example_1() -> ParseResult<'static, ()> {
    let input = r#"package MyPackage {
        import Protolog::*;

        block MyBlock1;

        block MyBlock2 {}
    }"#;
    let _ = parse_string(input)?;
    Ok(())
}
