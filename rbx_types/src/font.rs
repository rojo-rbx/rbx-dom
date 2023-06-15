#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Heavy,
}

impl FontWeight {
    pub fn from_u16(weight: u16) -> Option<Self> {
        Some(match weight {
            100 => FontWeight::Thin,
            200 => FontWeight::ExtraLight,
            300 => FontWeight::Light,
            400 => FontWeight::Regular,
            500 => FontWeight::Medium,
            600 => FontWeight::SemiBold,
            700 => FontWeight::Bold,
            800 => FontWeight::ExtraBold,
            900 => FontWeight::Heavy,
            _ => return None,
        })
    }
    pub fn as_u16(self) -> u16 {
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
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

impl FontStyle {
    pub fn from_u8(style: u8) -> Option<Self> {
        Some(match style {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            _ => return None,
        })
    }

    pub fn as_u8(self) -> u8 {
        match self {
            FontStyle::Normal => 0,
            FontStyle::Italic => 1,
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
    pub fn new(family: &str, weight: FontWeight, style: FontStyle) -> Self {
        Self {
            family: family.to_owned(),
            weight,
            style,
            cached_face_id: None,
        }
    }
    pub fn regular(family: &str) -> Self {
        Self {
            family: family.to_owned(),
            ..Default::default()
        }
    }
}
