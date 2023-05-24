use std::str::FromStr;

use serde::{Deserialize, Serialize};

// Tags found via:
// jq '[.Classes | .[] | .Members | .[] | select(.MemberType == "Property") | .Tags // empty] | add | unique' api-dump.json
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PropertyTag {
    Deprecated,
    Hidden,
    NotBrowsable,
    NotReplicated,
    NotScriptable,
    ReadOnly,
    WriteOnly,
}

#[derive(Debug)]
pub struct PropertyTagFromStrError(String);

impl FromStr for PropertyTag {
    type Err = PropertyTagFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "Deprecated" => Self::Deprecated,
            "Hidden" => Self::Hidden,
            "NotBrowsable" => Self::NotBrowsable,
            "NotReplicated" => Self::NotReplicated,
            "NotScriptable" => Self::NotScriptable,
            "ReadOnly" => Self::ReadOnly,
            "WriteOnly" => Self::WriteOnly,
            _ => return Err(PropertyTagFromStrError(value.to_owned())),
        })
    }
}
