/// Identifies the native representation that collapsed a target to null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum ReferenceKind {
    /// A direct Ref property lost its non-null document target.
    Ref,
    /// An Object entry in a Content property lost its non-null document target.
    ContentObject,
}
