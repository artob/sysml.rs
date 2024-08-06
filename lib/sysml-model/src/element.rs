// This is free and unencumbered software released into the public domain.

pub trait Element {
    /// The name of the element.
    fn name(&self) -> &str;

    /// The short name of the element, if any.
    fn short_name(&self) -> Option<&str>;
}
