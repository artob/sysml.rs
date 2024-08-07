// This is free and unencumbered software released into the public domain.

pub trait Element {
    /// The unique element ID, if any.
    fn id(&self) -> Option<&str> {
        None
    }

    /// The name of the element.
    fn name(&self) -> Option<&str> {
        None
    }

    /// The short name of the element, if any.
    fn short_name(&self) -> Option<&str> {
        None
    }
}
