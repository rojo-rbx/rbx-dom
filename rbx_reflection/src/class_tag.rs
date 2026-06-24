use std::str::FromStr;

use serde::{Deserialize, Serialize};

// Tags found via:
// jq '[.Classes | .[] | .Tags // empty] | add | unique' api-dump.json
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ClassTag {
    Deprecated,
    NotBrowsable,
    NotCreatable,
    NotReplicated,
    PlayerReplicated,
    Service,
    Settings,
    UserSettings,
}

#[derive(Debug)]
#[expect(dead_code)]
pub struct ClassTagFromStrError(String);

impl FromStr for ClassTag {
    type Err = ClassTagFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "Deprecated" => Self::Deprecated,
            "NotBrowsable" => Self::NotBrowsable,
            "NotCreatable" => Self::NotCreatable,
            "NotReplicated" => Self::NotReplicated,
            "PlayerReplicated" => Self::PlayerReplicated,
            "Service" => Self::Service,
            "Settings" => Self::Settings,
            "UserSettings" => Self::UserSettings,
            _ => return Err(ClassTagFromStrError(value.to_owned())),
        })
    }
}
