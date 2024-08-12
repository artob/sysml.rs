// This is free and unencumbered software released into the public domain.

use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
