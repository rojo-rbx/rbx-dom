use std::fmt;

/// Small utility to write formatting functions for lists of things.
///
/// This should be replaced by [`crate::lister::Lister`] once [PR #491] is merged
/// or it is fixed.
///
/// [PR #491]: https://github.com/rojo-rbx/rbx-dom/pull/491
pub(crate) struct Lister {
    first: bool,
}

impl Lister {
    pub fn new() -> Self {
        Self { first: true }
    }

    pub fn write(&mut self, out: &mut fmt::Formatter, label: impl fmt::Display) -> fmt::Result {
        if self.first {
            self.first = false;
            write!(out, "{}", label)
        } else {
            write!(out, ", {}", label)
        }
    }
}
