// This is free and unencumbered software released into the public domain.

use super::lexer::*;
use crate::{
    prelude::{vec, String, Vec},
    ParsedAttribute, ParsedBlock, ParsedImport, ParsedMember, ParsedModel, ParsedPackage,
    ParsedPort, Span, SyntaxResult,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    error::context,
    multi::many0,
    sequence::{delimited, preceded, terminated},
};
use sysml_model::QualifiedName;

pub fn model<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, ParsedModel)> {
    let (input, package) =
        context("model", delimited(multispace0, package, multispace0))(input.into())?;

    Ok((
        input,
        ParsedModel {
            members: vec![ParsedMember::Package(package)],
        },
    ))
}

pub fn members<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, Vec<ParsedMember>)> {
    let (input, members) = context(
        "tokens",
        many0(alt((
            terminated(map(import, ParsedMember::Import), multispace0),
            terminated(map(package, ParsedMember::Package), multispace0),
            terminated(map(block_usage, ParsedMember::BlockUsage), multispace0),
            terminated(
                map(attribute_usage, ParsedMember::AttributeUsage),
                multispace0,
            ),
            terminated(map(port_usage, ParsedMember::PortUsage), multispace0),
        ))),
    )(input.into())?;

    Ok((input, members))
}

pub fn import<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, ParsedImport)> {
    use nom::AsChar;
    let (input, _) = tag("import")(input.into())?;
    let (input, _) = multispace1(input)?;
    let (input, name) = map(take_while(|c| AsChar::as_char(c) != ';'), |span: Span| {
        QualifiedName::from(span.into_fragment())
    })(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, ParsedImport::new(name)))
}

pub fn package<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, ParsedPackage)> {
    let (input, (name, short_name, _, members)) = element(input.into(), "package")?;

    Ok((
        input,
        ParsedPackage {
            name: name.map(String::from),
            short_name: short_name.map(String::from),
            members,
        },
    ))
}

pub fn block_usage<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, ParsedBlock)> {
    let (input, (name, short_name, definition, members)) = element(input.into(), "block")?;

    Ok((
        input,
        ParsedBlock {
            name,
            short_name,
            definition,
            members,
        },
    ))
}

pub fn attribute_usage<'a>(
    input: impl Into<Span<'a>>,
) -> SyntaxResult<'a, (Span<'a>, ParsedAttribute)> {
    let (input, (name, short_name, definition, members)) = element(input.into(), "attribute")?;

    Ok((
        input,
        ParsedAttribute {
            name,
            short_name,
            definition,
            members,
        },
    ))
}

pub fn port_usage<'a>(input: impl Into<Span<'a>>) -> SyntaxResult<'a, (Span<'a>, ParsedPort)> {
    let (input, (name, short_name, definition, members)) = element(input.into(), "port")?;

    Ok((
        input,
        ParsedPort {
            name,
            short_name,
            definition,
            members,
        },
    ))
}

pub fn element<'a>(
    input: impl Into<Span<'a>>,
    tag_name: &'static str,
) -> SyntaxResult<
    'a,
    (
        Span<'a>,
        (
            Option<String>,
            Option<String>,
            Option<QualifiedName>,
            Vec<ParsedMember>,
        ),
    ),
> {
    let (input, _) = context(tag_name, tag(tag_name))(input.into())?;
    let (input, _) = multispace1(input)?;
    let (input, (name, short_name)) = identification(input)?;
    let (input, _) = multispace0(input)?;
    let (input, definition) = opt(delimited(
        terminated(char(':'), multispace0),
        qualified_name,
        multispace0,
    ))(input)?;
    let (input, members) = alt((
        map(char(';'), |_| Vec::new()),
        delimited(
            char('{'),
            preceded(multispace0, members),
            preceded(multispace0, char('}')),
        ),
    ))(input)?;

    Ok((input, (name, short_name, definition, members)))
}

pub fn identification<'a>(
    input: impl Into<Span<'a>>,
) -> SyntaxResult<'a, (Span<'a>, (Option<String>, Option<String>))> {
    let (input, short_name) = opt(delimited(char('<'), name, char('>')))(input.into())?;
    let (input, _) = multispace0(input)?;
    let (input, name) = opt(name)(input)?;

    Ok((input, (name, short_name)))
}

#[cfg(test)]
mod tests {}
