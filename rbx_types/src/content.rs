use crate::Ref;

/// A reference to a Roblox asset.
///
/// This is exposed in Roblox as either a string or the `Content` type depending
/// upon the property.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Content {
    value: ContentType,
}

/// An enum representing what type a `Content` is. Roblox may add new values to
/// this enum unexpectedly, so it is marked as `non_exhaustive`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ContentType {
    #[default]
    None,
    Uri(String),
    Object(Ref),
}

impl Content {
    /// Constructs an empty `Content`.
    #[inline]
    pub fn none() -> Self {
        Self {
            value: ContentType::None,
        }
    }

    /// Constructs a `Content` from the provided URI.
    pub fn from_uri<S: Into<String>>(uri: S) -> Self {
        Self {
            value: ContentType::Uri(uri.into()),
        }
    }

    /// Constructs a `Content` from the provided referent.
    #[inline]
    pub fn from_referent(referent: Ref) -> Self {
        Self {
            value: ContentType::Object(referent),
        }
    }

    #[inline]
    pub fn value(&self) -> &ContentType {
        &self.value
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut ContentType {
        &mut self.value
    }
}

impl From<String> for Content {
    fn from(url: String) -> Self {
        Self {
            value: ContentType::Uri(url),
        }
    }
}

impl From<&'_ str> for Content {
    fn from(url: &str) -> Self {
        Self {
            value: ContentType::Uri(url.to_owned()),
        }
    }
}
/// A reference to a Roblox asset.
///
/// When exposed to Lua, this is just a string.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]

pub struct ContentId {
    url: String,
}

impl ContentId {
    #[inline]
    pub fn new() -> Self {
        ContentId { url: String::new() }
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.url
    }
}

impl From<String> for ContentId {
    fn from(url: String) -> Self {
        Self { url }
    }
}

impl From<&'_ str> for ContentId {
    fn from(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

impl AsRef<str> for ContentId {
    fn as_ref(&self) -> &str {
        &self.url
    }
}

impl AsRef<String> for ContentId {
    fn as_ref(&self) -> &String {
        &self.url
    }
}

impl AsMut<str> for ContentId {
    fn as_mut(&mut self) -> &mut str {
        &mut self.url
    }
}

impl AsMut<String> for ContentId {
    fn as_mut(&mut self) -> &mut String {
        &mut self.url
    }
}
