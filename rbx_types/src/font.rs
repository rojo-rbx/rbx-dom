#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Heavy,
    Other(u16),
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Regular
    }
}

impl From<u16> for FontWeight {
    fn from(weight: u16) -> Self {
        match weight {
            100 => FontWeight::Thin,
            200 => FontWeight::ExtraLight,
            300 => FontWeight::Light,
            400 => FontWeight::Regular,
            500 => FontWeight::Medium,
            600 => FontWeight::SemiBold,
            700 => FontWeight::Bold,
            800 => FontWeight::ExtraBold,
            900 => FontWeight::Heavy,
            other => FontWeight::Other(other),
        }
    }
}

impl From<FontWeight> for u16 {
    fn from(weight: FontWeight) -> Self {
        match weight {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Heavy => 900,
            FontWeight::Other(other) => other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum FontStyle {
    Normal,
    Italic,
    Other(u8),
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

impl From<u8> for FontStyle {
    fn from(style: u8) -> Self {
        match style {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            other => FontStyle::Other(other),
        }
    }
}

impl From<FontStyle> for u8 {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => 0,
            FontStyle::Italic => 1,
            FontStyle::Other(other) => other,
        }
    }
}

/// A font face consisting of a typeface and other style properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Font {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub cached_face_id: Option<String>,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "rbxasset://fonts/families/SourceSansPro.json".to_owned(),
            weight: FontWeight::default(),
            style: FontStyle::default(),
            cached_face_id: None,
        }
    }
}

impl Font {
    #[inline]
    pub fn new(
        family: String,
        weight: FontWeight,
        style: FontStyle,
        cached_face_id: Option<String>,
    ) -> Self {
        Font {
            family,
            weight,
            style,
            cached_face_id,
        }
    }
}
