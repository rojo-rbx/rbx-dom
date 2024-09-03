use std::string::FromUtf8Error;

/// Contains a list of tags that can be applied to an instance.
///
/// This object does not ensure that tags are unique; there may be duplicate
/// values in the list of tags.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Tags {
    // Future improvement: use a single String to hold all tags, delimited by
    // `\0` like Roblox does for serialization.
    members: Vec<String>,
}

impl Tags {
    /// Create a new `Tags` empty container.
    pub fn new() -> Tags {
        Self {
            members: Vec::new(),
        }
    }

    /// Add a tag to the list of tags in the container.
    pub fn push(&mut self, tag: &str) {
        self.members.push(tag.to_owned());
    }

    /// Returns an iterator over all of the tags in the container.
    pub fn iter(&self) -> TagsIter<'_> {
        TagsIter {
            internal: self.members.iter(),
        }
    }

    /// Decodes tags from a buffer containing `\0`-delimited tag names.
    pub fn decode(buf: &[u8]) -> Result<Self, FromUtf8Error> {
        Ok(buf
            .split(|element| *element == 0)
            .filter(|tag_name| !tag_name.is_empty())
            .map(|tag_name| String::from_utf8(tag_name.to_vec()))
            .collect::<Result<Vec<String>, _>>()?
            .into())
    }

    /// Encodes tags into a buffer by joining them with `\0` bytes.
    pub fn encode(&self) -> Vec<u8> {
        self.members.join("\0").into_bytes()
    }

    /// Returns the number of strings stored within this `Tags`.
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns `true` if this `Tags` contains no strings.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

impl From<Vec<String>> for Tags {
    fn from(members: Vec<String>) -> Tags {
        Self { members }
    }
}

/// See [`Tags::iter`].
pub struct TagsIter<'a> {
    internal: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for TagsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next().map(|v| v.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn serialization() {
        let serialized = r#"["foo","grandma's","coat?","bar"]"#;
        let expected = Tags::from(vec![
            "foo".to_owned(),
            "grandma's".to_owned(),
            "coat?".to_owned(),
            "bar".to_owned(),
        ]);

        assert_eq!(serialized, serde_json::to_string(&expected).unwrap());
        assert_eq!(expected, serde_json::from_str::<Tags>(serialized).unwrap());
    }

    #[test]
    fn decode_encode() {
        let value = b"ez\0pz";
        let tags = Tags::decode(value).unwrap();

        assert_eq!(tags.iter().collect::<Vec<_>>(), &["ez", "pz"]);
        assert_eq!(tags.encode(), value);
    }

    #[test]
    fn decode_empty() {
        let input = b"";
        let expected: &[String] = &[];
        let result = Tags::decode(input).unwrap();

        assert_eq!(result.iter().collect::<Vec<_>>(), expected);
    }
}
