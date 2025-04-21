use crate::Ref;

/// A reference to a Roblox asset.
///
/// This is exposed in Roblox as the `Content` type. For the legacy type equivalent
/// to a string, see `ContentId`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Content(ContentType);

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
        Self(ContentType::None)
    }

    /// Constructs a `Content` from the provided URI.
    pub fn from_uri<S: Into<String>>(uri: S) -> Self {
        Self(ContentType::Uri(uri.into()))
    }

    /// Constructs a `Content` from the provided referent.
    #[inline]
    pub fn from_referent(referent: Ref) -> Self {
        Self(ContentType::Object(referent))
    }

    /// Returns the underlying value of the `Content`.
    #[inline]
    pub fn value(&self) -> &ContentType {
        &self.0
    }

    /// Returns a mutable reference to the underlying value of the `Content`.
    #[inline]
    pub fn value_mut(&mut self) -> &mut ContentType {
        &mut self.0
    }

    /// Consumes this `Content` and returns the underlying value.
    #[inline]
    pub fn into_value(self) -> ContentType {
        self.0
    }
}

impl From<String> for Content {
    fn from(url: String) -> Self {
        Self(ContentType::Uri(url))
    }
}

impl From<&'_ str> for Content {
    fn from(url: &str) -> Self {
        Self(ContentType::Uri(url.to_owned()))
    }
}
/// A reference to a Roblox asset.
///
/// When exposed to Luau, this is just a string. For the modern userdata type,
/// see `Content`.
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
    /// Constructs an empty new `ContentId`.
    #[inline]
    pub fn new() -> Self {
        ContentId { url: String::new() }
    }

    /// Returns the `ContentId`'s value as a `&str`.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.url
    }

    /// Converts this `ContentId` into a `String`.
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
