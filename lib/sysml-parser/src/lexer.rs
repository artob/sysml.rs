// This is free and unencumbered software released into the public domain.

use super::Keyword;
use crate::prelude::String;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{map, recognize, verify},
    multi::many0,
    sequence::{delimited, pair},
    IResult
};
use sysml_model::QualifiedName;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Token {
    Name(String),
    QualifiedName(QualifiedName),
    Keyword(Keyword),
}

pub fn name(input: &str) -> IResult<&str, Token> {
    alt((basic_name, unrestricted_name))(input)
}

pub fn basic_name(input: &str) -> IResult<&str, Token> {
    let (input, name) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)?;

    Ok((input, Token::Name(String::from(name))))
}

pub fn unrestricted_name(input: &str) -> IResult<&str, Token> {
    let (input, name) = delimited(char('\''), is_not("'"), char('\''))(input)?;

    Ok((input, Token::Name(String::from(name))))
}

pub fn reserved_keyword(input: &str) -> IResult<&str, Keyword> {
    alt((
        map(keyword("about"), |_| Keyword::About),
        map(keyword("abstract"), |_| Keyword::Abstract),
        map(keyword("accept"), |_| Keyword::Accept),
        map(keyword("action"), |_| Keyword::Action),
        map(keyword("actor"), |_| Keyword::Actor),
        map(keyword("after"), |_| Keyword::After),
        map(keyword("alias"), |_| Keyword::Alias),
        map(keyword("all"), |_| Keyword::All),
        map(keyword("allocate"), |_| Keyword::Allocate),
        map(keyword("allocation"), |_| Keyword::Allocation),
        map(keyword("analysis"), |_| Keyword::Analysis),
        map(keyword("and"), |_| Keyword::And),
        map(keyword("as"), |_| Keyword::As),
        map(keyword("assign"), |_| Keyword::Assign),
        map(keyword("assert"), |_| Keyword::Assert),
        map(keyword("assoc"), |_| Keyword::Assoc),
        map(keyword("assume"), |_| Keyword::Assume),
        map(keyword("at"), |_| Keyword::At),
        map(keyword("attribute"), |_| Keyword::Attribute),
        map(keyword("bind"), |_| Keyword::Bind),
    alt((
        map(keyword("binding"), |_| Keyword::Binding),
        map(keyword("block"), |_| Keyword::Block),
        map(keyword("by"), |_| Keyword::By),
        map(keyword("calc"), |_| Keyword::Calc),
        map(keyword("case"), |_| Keyword::Case),
        map(keyword("comment"), |_| Keyword::Comment),
        map(keyword("concern"), |_| Keyword::Concern),
        map(keyword("connect"), |_| Keyword::Connect),
        map(keyword("connection"), |_| Keyword::Connection),
        map(keyword("constraint"), |_| Keyword::Constraint),
        map(keyword("decide"), |_| Keyword::Decide),
        map(keyword("def"), |_| Keyword::Def),
        map(keyword("default"), |_| Keyword::Default),
        map(keyword("defined"), |_| Keyword::Defined),
        map(keyword("dependency"), |_| Keyword::Dependency),
        map(keyword("derived"), |_| Keyword::Derived),
        map(keyword("do"), |_| Keyword::Do),
        map(keyword("doc"), |_| Keyword::Doc),
        map(keyword("else"), |_| Keyword::Else),
        map(keyword("end"), |_| Keyword::End),
    alt((
        map(keyword("entry"), |_| Keyword::Entry),
        map(keyword("enum"), |_| Keyword::Enum),
        map(keyword("event"), |_| Keyword::Event),
        map(keyword("exhibit"), |_| Keyword::Exhibit),
        map(keyword("exit"), |_| Keyword::Exit),
        map(keyword("expose"), |_| Keyword::Expose),
        map(keyword("filter"), |_| Keyword::Filter),
        map(keyword("first"), |_| Keyword::First),
        map(keyword("flow"), |_| Keyword::Flow),
        map(keyword("for"), |_| Keyword::For),
        map(keyword("fork"), |_| Keyword::Fork),
        map(keyword("frame"), |_| Keyword::Frame),
        map(keyword("from"), |_| Keyword::From),
        map(keyword("hastype"), |_| Keyword::HasType),
        map(keyword("if"), |_| Keyword::If),
        map(keyword("implies"), |_| Keyword::Implies),
        map(keyword("import"), |_| Keyword::Import),
        map(keyword("in"), |_| Keyword::In),
        map(keyword("include"), |_| Keyword::Include),
        map(keyword("individual"), |_| Keyword::Individual),
    alt((
        map(keyword("inout"), |_| Keyword::InOut),
        map(keyword("interface"), |_| Keyword::Interface),
        map(keyword("istype"), |_| Keyword::IsType),
        map(keyword("item"), |_| Keyword::Item),
        map(keyword("join"), |_| Keyword::Join),
        map(keyword("language"), |_| Keyword::Language),
        map(keyword("loop"), |_| Keyword::Loop),
        map(keyword("merge"), |_| Keyword::Merge),
        map(keyword("message"), |_| Keyword::Message),
        map(keyword("metadata"), |_| Keyword::Metadata),
        map(keyword("nonunique"), |_| Keyword::NonUnique),
        map(keyword("not"), |_| Keyword::Not),
        map(keyword("objective"), |_| Keyword::Objective),
        map(keyword("occurrence"), |_| Keyword::Occurrence),
        map(keyword("of"), |_| Keyword::Of),
        map(keyword("or"), |_| Keyword::Or),
        map(keyword("ordered"), |_| Keyword::Ordered),
        map(keyword("out"), |_| Keyword::Out),
        map(keyword("package"), |_| Keyword::Package),
        map(keyword("parallel"), |_| Keyword::Parallel),
    alt((
        map(keyword("part"), |_| Keyword::Part),
        map(keyword("perform"), |_| Keyword::Perform),
        map(keyword("port"), |_| Keyword::Port),
        map(keyword("private"), |_| Keyword::Private),
        map(keyword("protected"), |_| Keyword::Protected),
        map(keyword("public"), |_| Keyword::Public),
        map(keyword("readonly"), |_| Keyword::ReadOnly),
        map(keyword("redefines"), |_| Keyword::Redefines),
        map(keyword("ref"), |_| Keyword::Ref),
        map(keyword("references"), |_| Keyword::References),
        map(keyword("render"), |_| Keyword::Render),
        map(keyword("rendering"), |_| Keyword::Rendering),
        map(keyword("rep"), |_| Keyword::Rep),
        map(keyword("require"), |_| Keyword::Require),
        map(keyword("requirement"), |_| Keyword::Requirement),
        map(keyword("return"), |_| Keyword::Return),
        map(keyword("satisfy"), |_| Keyword::Satisfy),
        map(keyword("send"), |_| Keyword::Send),
        map(keyword("snapshot"), |_| Keyword::Snapshot),
        map(keyword("specializes"), |_| Keyword::Specializes),
    alt((
        map(keyword("stakeholder"), |_| Keyword::Stakeholder),
        map(keyword("state"), |_| Keyword::State),
        map(keyword("subject"), |_| Keyword::Subject),
        map(keyword("subsets"), |_| Keyword::Subsets),
        map(keyword("succession"), |_| Keyword::Succession),
        map(keyword("then"), |_| Keyword::Then),
        map(keyword("timeslice"), |_| Keyword::TimeSlice),
        map(keyword("to"), |_| Keyword::To),
        map(keyword("transition"), |_| Keyword::Transition),
        map(keyword("until"), |_| Keyword::Until),
        map(keyword("use"), |_| Keyword::Use),
        map(keyword("variant"), |_| Keyword::Variant),
        map(keyword("variation"), |_| Keyword::Variation),
        map(keyword("verification"), |_| Keyword::Verification),
        map(keyword("verify"), |_| Keyword::Verify),
        map(keyword("via"), |_| Keyword::Via),
        map(keyword("view"), |_| Keyword::View),
        map(keyword("viewpoint"), |_| Keyword::Viewpoint),
        map(keyword("when"), |_| Keyword::When),
        map(keyword("while"), |_| Keyword::While),
    alt((
        map(keyword("xor"), |_| Keyword::Xor),
    ))))))))))))))(input)
}

fn keyword<'a>(word: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        verify(tag(word), |s: &str| {
            !input[s.len()..].chars().next().map_or(false, |c| c.is_alphanumeric() || c == '_')
        })(input)
    }
}

#[cfg(test)]
mod tests {}
