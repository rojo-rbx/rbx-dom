use std::fmt;

use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

/// A unique ID that represents an instance within an [`RbxTree`].
///
/// rbx_dom_weak uses UUIDv4 values for instance IDs and serializes equivalently.
///
/// [`RbxTree`]: struct.RbxTree.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RbxId(Uuid);

// Default randomly generated stuff doesn't make sense.
#[allow(clippy::new_without_default)]
impl RbxId {
    /// Generates a new, random `RbxId`.
    pub fn new() -> RbxId {
        RbxId(Uuid::new_v4())
    }

    /// Parses an `RbxId` from a string containing a UUID.
    pub fn parse_str(input: &str) -> Option<RbxId> {
        Uuid::parse_str(input).map(RbxId).ok()
    }
}

impl fmt::Display for RbxId {
    fn fmt(&self, writer: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(writer, "{}", self.0)
    }
}
