use std::fmt;

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct SecurityCapabilities {
    value: u64,
}

impl SecurityCapabilities {
    pub fn from_bits(value: u64) -> Self {
        SecurityCapabilities { value }
    }
}

impl fmt::Display for SecurityCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}
