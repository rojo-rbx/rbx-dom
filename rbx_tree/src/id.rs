use std::fmt;

use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RbxId(Uuid);

impl RbxId {
    pub fn new() -> RbxId {
        RbxId(Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Option<RbxId> {
        Uuid::parse_str(input).map(RbxId).ok()
    }
}

impl fmt::Display for RbxId {
    fn fmt(&self, writer: &mut fmt::Formatter) -> fmt::Result {
        write!(writer, "{}", self.0)
    }
}