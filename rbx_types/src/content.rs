/// A reference to a Roblox asset.
///
/// When exposed to Lua, this is just a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Content {
    url: String,
}

impl Content {
    #[inline]
    pub fn new() -> Self {
        Content { url: String::new() }
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.url
    }
}

impl From<String> for Content {
    fn from(url: String) -> Self {
        Self { url }
    }
}

impl From<&'_ str> for Content {
    fn from(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

impl AsRef<str> for Content {
    fn as_ref(&self) -> &str {
        &self.url
    }
}

impl AsRef<String> for Content {
    fn as_ref(&self) -> &String {
        &self.url
    }
}

impl AsMut<str> for Content {
    fn as_mut(&mut self) -> &mut str {
        &mut self.url
    }
}

impl AsMut<String> for Content {
    fn as_mut(&mut self) -> &mut String {
        &mut self.url
    }
}
