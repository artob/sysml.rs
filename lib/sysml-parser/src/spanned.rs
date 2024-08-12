// This is free and unencumbered software released into the public domain.

use crate::span::Span;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Spanned<'a, T> {
    pub value: T,
    pub span: Span<'a>,
}

impl<'a, T> Spanned<'a, T> {
    pub fn new(value: T, span: Span<'a>) -> Self {
        Self { value, span }
    }
}
