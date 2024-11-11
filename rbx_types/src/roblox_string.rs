use std::fmt::Display;

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RobloxString {
    Binary(Vec<u8>),
    Utf8(String),
}

impl RobloxString {
    pub fn new() -> Self {
        Self::Utf8(String::new())
    }
}

impl Default for RobloxString {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for RobloxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Utf8(string) => string.to_string(),
            Self::Binary(buffer) => String::from_utf8_lossy(buffer).to_string(),
        };

        f.write_str(output.as_str())
    }
}

impl From<Vec<u8>> for RobloxString {
    fn from(buffer: Vec<u8>) -> Self {
        if let Ok(string) = std::str::from_utf8(buffer.as_slice()) {
            Self::Utf8(string.to_string())
        } else {
            Self::Binary(buffer)
        }
    }
}

impl From<RobloxString> for Vec<u8> {
    fn from(value: RobloxString) -> Self {
        match value {
            RobloxString::Binary(buffer) => buffer,
            RobloxString::Utf8(string) => string.into_bytes(),
        }
    }
}

impl From<&[u8]> for RobloxString {
    fn from(buffer: &[u8]) -> Self {
        if let Ok(string) = std::str::from_utf8(buffer) {
            Self::Utf8(string.to_string())
        } else {
            Self::Binary(buffer.to_vec())
        }
    }
}

impl AsRef<[u8]> for RobloxString {
    fn as_ref(&self) -> &[u8] {
        match self {
            RobloxString::Binary(buffer) => buffer.as_slice(),
            RobloxString::Utf8(string) => string.as_bytes(),
        }
    }
}

impl From<String> for RobloxString {
    fn from(value: String) -> Self {
        Self::Utf8(value)
    }
}

impl From<&String> for RobloxString {
    fn from(value: &String) -> Self {
        Self::Utf8(value.to_string())
    }
}

impl From<&str> for RobloxString {
    fn from(value: &str) -> Self {
        Self::Utf8(value.to_string())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{ser::SerializeTupleVariant, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for RobloxString {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                RobloxString::Binary(buffer) => {
                    let is_human_readable = serializer.is_human_readable();
                    let mut tv =
                        serializer.serialize_tuple_variant("RobloxString", 0, "Binary", 1)?;

                    if is_human_readable {
                        let encoded = base64::encode(buffer);
                        tv.serialize_field(&encoded)?
                    } else {
                        tv.serialize_field(buffer)?
                    };

                    tv.end()
                }
                RobloxString::Utf8(string) => {
                    let mut tv =
                        serializer.serialize_tuple_variant("RobloxString", 1, "Utf8", 1)?;

                    tv.serialize_field(string)?;
                    tv.end()
                }
            }
        }
    }

    impl<'de> Deserialize<'de> for RobloxString {
        fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
            todo!()
        }
    }
}
