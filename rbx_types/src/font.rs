/// Represents a 'weight' for a Font. These correspond to values of the
/// [`FontWeight`][FontWeight-rbx] enum on Roblox.
///
/// Due to the nature of Roblox enums, this is marked as non-exhaustive as
/// Roblox may add more variants to it at any time.
///
/// [FontWeight-rbx]: https://create.roblox.com/docs/reference/engine/enums/FontWeight
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
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
    /// Creates a `FontWeight` from the provided value, if possible. Accepted
    /// values correspond to entries in the [`FontWeight`][FontWeight-rbx] enum
    /// on Roblox.
    ///
    /// [FontWeight-rbx]: https://create.roblox.com/docs/reference/engine/enums/FontWeight
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

    /// Converts this vaue into a number identifier. Values are converted into
    /// their respective value of the Roblox [`FontWeight`][FontWeight-rbx]
    /// enum. As an example, `FontWeight::Thin` is converted to `100`.
    ///
    /// [FontWeight-rbx]: https://create.roblox.com/docs/reference/engine/enums/FontWeight
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

/// Represents a 'style' for a Font. These correspond to values of the
/// [`FontStyle`][FontStyle-rbx] enum on Roblox.
///
/// Due to the nature of Roblox enums, this is marked as non-exhaustive as
/// Roblox may add more variants to it at any time.
///
/// [FontStyle-rbx]: https://create.roblox.com/docs/reference/engine/enums/FontStyle
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

impl FontStyle {
    /// Creates a `FontStyle` from the provided value, if possible. Accepted
    /// values correspond to entries in the [`FontStyle`][FontStyle-rbx] enum
    /// on Roblox.
    ///
    /// [FontStyle-rbx]: https://create.roblox.com/docs/reference/engine/enums/FontStyle
    pub fn from_u8(style: u8) -> Option<Self> {
        Some(match style {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            _ => return None,
        })
    }

    /// Converts this vaue into a number identifier. Values are converted into
    /// their respective value of the Roblox [`FontStyle`][FontStyle-rbx]
    /// enum. As an example, `FontStyle::Normal` is converted to `0`.
    ///
    /// [FontStyle-rbx]: https://create.roblox.com/docs/reference/engine/enums/FontStyle
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
    /// Constructs a new font with the provided family, weight, and style.
    pub fn new(family: &str, weight: FontWeight, style: FontStyle) -> Self {
        Self {
            family: family.to_owned(),
            weight,
            style,
            cached_face_id: None,
        }
    }

    /// Constructs a 'regular' font from the provided family, with the default
    /// weight and style.
    pub fn regular(family: &str) -> Self {
        Self {
            family: family.to_owned(),
            ..Default::default()
        }
    }
}
