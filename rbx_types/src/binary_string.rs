/// Container for untyped binary data.
///
/// `BinaryString` is used in cases where the type of the underlying data is
/// unknown or unimplemented. Where possible, stronger types that interpret the
/// underlying bytes should be preferred.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BinaryString {
    buffer: Vec<u8>,
}

impl BinaryString {
    #[inline]
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }
}

impl From<Vec<u8>> for BinaryString {
    fn from(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }
}

impl From<&'_ [u8]> for BinaryString {
    fn from(buffer: &[u8]) -> Self {
        Self {
            buffer: buffer.to_vec(),
        }
    }
}

impl From<BinaryString> for Vec<u8> {
    fn from(value: BinaryString) -> Self {
        value.buffer
    }
}

impl AsRef<[u8]> for BinaryString {
    fn as_ref(&self) -> &[u8] {
        &self.buffer
    }
}

impl AsRef<Vec<u8>> for BinaryString {
    fn as_ref(&self) -> &Vec<u8> {
        &self.buffer
    }
}

impl AsMut<[u8]> for BinaryString {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

impl AsMut<Vec<u8>> for BinaryString {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for BinaryString {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                let encoded = base64::encode(&self.buffer);

                serializer.serialize_str(&encoded)
            } else {
                // We need to be opaque here because we're deserializing
                // using `Vec<u8>`'s serde implementation and we cannot trust
                // that it'll be implemented the same across versions
                self.buffer.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for BinaryString {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                let encoded = <&str>::deserialize(deserializer)?;
                let buffer = base64::decode(encoded).map_err(D::Error::custom)?;

                Ok(BinaryString { buffer })
            } else {
                let buffer = <Vec<u8>>::deserialize(deserializer)?;
                Ok(BinaryString { buffer })
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human() {
        let data = BinaryString::from(b"hello".to_vec());

        let ser = serde_json::to_string(&data).unwrap();
        assert_eq!(ser, "\"aGVsbG8=\"");

        let de: BinaryString = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, data);
    }

    #[test]
    fn non_human() {
        let data = BinaryString::from(b"world".to_vec());

        let ser = bincode::serialize(&data).unwrap();
        let de: BinaryString = bincode::deserialize(&ser).unwrap();

        assert_eq!(de, data);
    }
}
