// This is free and unencumbered software released into the public domain.

use crate::{grammar::package, lexer::Span, prelude::Rc, ParseResult};
use nom::character::complete::multispace0;
use sysml_model::Package;

pub fn parse_from_string(input: &str) -> ParseResult<Rc<dyn Package>> {
    let input = Span::new(input);
    let (input, _) = multispace0(input)?;
    let (_, package) = package(input)?;
    Ok(Rc::new(package))
}
