/// Identifies the native representation that collapsed a target to null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    /// A direct Ref property lost its non-null document target.
    Ref,
    /// An Object entry in a Content property lost its non-null document target.
    ContentObject,
}
