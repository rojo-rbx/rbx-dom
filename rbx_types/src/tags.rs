use core::str::Utf8Error;

/// Contains a list of tags that can be applied to an instance.
///
/// This object does not ensure that tags are unique; there may be duplicate
/// values in the list of tags.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Tags {
    members: String,
}

impl Tags {
    /// Create a new `Tags` empty container.
    pub const fn new() -> Tags {
        Self {
            members: String::new(),
        }
    }

    /// Add a tag to the list of tags in the container.
    pub fn push(&mut self, tag: &str) {
        self.members.push_str(tag);
        self.members.push('\0');
    }

    /// Returns an iterator over non-empty tags in the container.
    pub fn iter(&self) -> TagsIter<'_> {
        TagsIter::new(&self.members)
    }

    /// Decodes tags from a buffer containing `\0`-delimited tag names.
    pub fn decode(buf: &[u8]) -> Result<Self, Utf8Error> {
        let str = str::from_utf8(buf)?;
        // trim '\0's from beginning and end
        let trimmed = str.trim_matches('\0');
        if trimmed.is_empty() {
            let members = String::new();
            return Ok(Self { members });
        }
        let mut members = String::with_capacity(trimmed.len() + 1);
        members.push_str(trimmed);
        // encode expects '\0' postfix
        members.push('\0');
        Ok(Self { members })
    }

    /// Get the encoded representation of the tags
    pub fn encode(&self) -> &[u8] {
        // Chop off '\0' postfix when it exists
        let postfix_offset = !self.is_empty() as usize;
        &self.members.as_bytes()[..self.members.len() - postfix_offset]
    }

    /// Returns the number of strings stored within this `Tags`.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns `true` if this `Tags` contains no strings.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

impl<'a> core::iter::FromIterator<&'a str> for Tags {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut tags = Tags::new();
        for tag in iter {
            tags.push(tag);
        }
        tags
    }
}
impl<'a> IntoIterator for &'a Tags {
    type Item = &'a str;
    type IntoIter = TagsIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over non-empty tags.
pub struct TagsIter<'a> {
    internal: core::str::Split<'a, char>,
}
impl<'a> TagsIter<'a> {
    fn new(members: &'a str) -> Self {
        TagsIter {
            internal: members.split('\0'),
        }
    }
}

impl<'a> Iterator for TagsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.find(|tag| !tag.is_empty())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;
        for tag in self {
            seq.serialize_element(tag)?;
        }
        seq.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Tags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TagsVisitor;
        impl<'de> serde::de::Visitor<'de> for TagsVisitor {
            type Value = Tags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a Tags value")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut tags = Tags::new();
                while let Some(tag) = seq.next_element()? {
                    tags.push(tag);
                }
                Ok(tags)
            }
        }

        deserializer.deserialize_seq(TagsVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn serialization() {
        use core::iter::FromIterator;

        let serialized = r#"["foo","grandma's","coat?","bar"]"#;
        let expected = Tags::from_iter(["foo", "grandma's", "coat?", "bar"]);

        assert_eq!(serialized, serde_json::to_string(&expected).unwrap());
        assert_eq!(expected, serde_json::from_str::<Tags>(serialized).unwrap());
    }

    #[test]
    fn decode_encode() {
        macro_rules! test {
            ($input:expr, $expected_encoded:expr, $expected_array:expr) => {
                let tags = Tags::decode($input).unwrap();
                assert_eq!(
                    tags.encode(),
                    $expected_encoded,
                    "encoded tags do not match expected"
                );
                let mut tags_iter = tags.iter();
                let array: [&str; _] = core::array::from_fn(|_| tags_iter.next().unwrap());
                let expected_array: [&str; _] = $expected_array;
                assert_eq!(array, expected_array, "decoded tags do not match expected");
                assert!(
                    tags_iter.next().is_none(),
                    "TagsIter yielded too many items"
                );
            };
        }

        // decode input, expected encode, expected iter
        test!(b"", b"", []);
        test!(b"\0", b"", []);
        test!(b"\0ez", b"ez", ["ez"]);
        test!(b"\0ez\0", b"ez", ["ez"]);
        test!(b"ez", b"ez", ["ez"]);
        test!(b"ez\0", b"ez", ["ez"]);
        test!(b"ez\0pz", b"ez\0pz", ["ez", "pz"]);
        test!(b"ez\0\0pz", b"ez\0\0pz", ["ez", "pz"]);
        test!(b"ez\0\0\0pz", b"ez\0\0\0pz", ["ez", "pz"]);
    }
}
