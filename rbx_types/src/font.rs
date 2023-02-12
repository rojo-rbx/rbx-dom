#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl FontWeight {
    pub fn from_u16(weight: u16) -> Self {
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
    pub fn to_u16(self) -> u16 {
        match self {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

impl FontStyle {
    pub fn from_u8(style: u8) -> Self {
        match style {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            other => FontStyle::Other(other),
        }
    }
    pub fn to_u8(self) -> u8 {
        match self {
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
