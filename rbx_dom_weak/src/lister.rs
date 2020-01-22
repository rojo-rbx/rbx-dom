use std::fmt;

/// Small utility to write formatting functions for lists of things.
pub(crate) struct Lister {
    first: bool,
}

impl Lister {
    pub fn new() -> Self {
        Self { first: true }
    }

    pub fn write(&mut self, out: &mut fmt::Formatter, label: impl fmt::Display) -> fmt::Result {
        if !self.first {
            write!(out, ", ")?;
        }

        write!(out, "{}", label)
    }
}
