use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use thiserror::Error;

use std::{
    convert::TryFrom,
    fmt,
    sync::atomic::{AtomicU32, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// The `UniqueId` epoch (2021-01-01 00:00:00 GMT) in terms of time since the Unix epoch
const EPOCH_AS_UNIX: u64 = 1_609_459_200;

lazy_static! {
    /// A `SystemTime` representing the `UniqueId` epoch.
    pub static ref EPOCH: SystemTime = UNIX_EPOCH - Duration::from_secs(EPOCH_AS_UNIX);
}

/// Represents an error that can occur when constructing a new `UniqueId`.
#[derive(Debug, Error)]
pub enum UniqueIdError {
    #[error("SystemTime generated a timestamp that is before the UniqueId epoch")]
    SystemPastTime,
    #[error("UniqueId timestamp is more than 2^32 - 1 seconds past epoch")]
    Overflow,
}

/// Represents a UUID with a custom epoch of midnight January 1st 2021.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniqueId {
    index: u32,
    time: u32,
    random: i64,
}

static INDEX: AtomicU32 = AtomicU32::new(0);

impl UniqueId {
    pub fn new(index: u32, time: u32, random: i64) -> Self {
        Self {
            index,
            time,
            random,
        }
    }

    pub fn now() -> Result<Self, UniqueIdError> {
        let time = SystemTime::now()
            .duration_since(*EPOCH)
            .map_err(|_| UniqueIdError::SystemPastTime)?;

        Ok(Self {
            index: INDEX.fetch_add(1, Ordering::AcqRel),
            time: u32::try_from(time.as_secs()).map_err(|_| UniqueIdError::Overflow)?,
            // This matches Roblox's behavior, where the value is both an i64
            // but is also always positive.
            random: thread_rng().gen_range(0..i64::MAX),
        })
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn random(&self) -> i64 {
        self.random
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}{:08x}{:08x}", self.random, self.time, self.index)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::UniqueId;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    use std::{convert::TryInto, fmt};

    impl Serialize for UniqueId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                serializer.serialize_str(&self.to_string())
            } else {
                let mut bytes = Vec::with_capacity(16);
                bytes.extend_from_slice(&self.random.to_be_bytes());
                bytes.extend_from_slice(&self.time.to_be_bytes());
                bytes.extend_from_slice(&self.index.to_be_bytes());

                serializer.serialize_bytes(&bytes)
            }
        }
    }

    impl<'de> Deserialize<'de> for UniqueId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(HumanVisitor)
            } else {
                deserializer.deserialize_bytes(NonHumanVisitor)
            }
        }
    }

    struct HumanVisitor;
    struct NonHumanVisitor;

    impl<'de> de::Visitor<'de> for HumanVisitor {
        type Value = UniqueId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a sequence of 32 hexadecimal characters")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() == 32 {
                Ok(UniqueId {
                    random: i64::from_str_radix(&v[0..16], 16).map_err(E::custom)?,
                    time: u32::from_str_radix(&v[16..24], 16).map_err(E::custom)?,
                    index: u32::from_str_radix(&v[24..32], 16).map_err(E::custom)?,
                })
            } else {
                Err(E::custom(format!(
                    "invalid length of hex string: {}",
                    v.len()
                )))
            }
        }
    }

    impl<'de> de::Visitor<'de> for NonHumanVisitor {
        type Value = UniqueId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a sequence of 16 bytes")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() == 16 {
                Ok(UniqueId {
                    random: i64::from_be_bytes(v[0..8].try_into().unwrap()),
                    time: u32::from_be_bytes(v[8..12].try_into().unwrap()),
                    index: u32::from_be_bytes(v[12..16].try_into().unwrap()),
                })
            } else {
                Err(E::custom(format!(
                    "invalid length of byte sequence: {}",
                    v.len()
                )))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::UniqueId;

    #[test]
    fn display() {
        let uid = UniqueId::new(0xdead_beef, 0x0013_3700, 0x0badf00dc0ffee42);
        assert_eq!(uid.to_string(), "0badf00dc0ffee4200133700deadbeef");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn human_roundtrip() {
        let uid = UniqueId::new(0x1337_0000, 0xfaca_de00, 0x1020_3040_5060_7080);
        let ser = serde_json::to_string(&uid).unwrap();
        let de: UniqueId = serde_json::from_str(&ser).unwrap();

        assert_eq!(ser, r#""1020304050607080facade0013370000""#);
        assert_eq!(de, uid);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn not_human_roundtrip() {
        let uid = UniqueId::new(0x1337_0000, 0xfaca_de00, 0x1020_3040_5060_7080);
        let ser = bincode::serialize(&uid).unwrap();
        let de: UniqueId = bincode::deserialize(&ser).unwrap();

        // Bincode prefixes vectors with the vector's length as a little-endian `u64`
        assert_eq!(ser[0..8].as_ref(), 16_u64.to_le_bytes());

        assert_eq!(
            ser[8..].as_ref(),
            b"\x10\x20\x30\x40\x50\x60\x70\x80\xfa\xca\xde\x00\x13\x37\x00\x00"
        );
        assert_eq!(de, uid);
    }
}
