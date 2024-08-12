// This is free and unencumbered software released into the public domain.

use sysml_parser::{grammar::package, SyntaxResult};

#[test]
fn parse_example_1() -> SyntaxResult<'static, ()> {
    let input = r#"package MySystem {
        import Protoflow::*;

        block source : Random::U64;

        block delay : Delay::U64 {
            attribute delay : Duration;
        }

        block sink : Buffer::U64;
    }"#;
    let _system = package(input.into())?.1;
    //std::eprintln!("{:?}", _system);
    Ok(())
}
