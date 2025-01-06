use std::{collections::BTreeMap, str::FromStr};

use thiserror::Error;

use crate::Color3uint8;

use crate::Error as CrateError;

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
/// Represents the mapping of materials to colors used by Roblox's `Terrain`.
pub struct MaterialColors {
    /// The underlying map used by this struct. A `BTreeMap` is used
    /// over a `HashMap` to ensure serialization with serde is ordered.
    inner: BTreeMap<TerrainColorMaterial, Color3uint8>,
}

impl MaterialColors {
    /// Constructs a new `MaterialColors` where all colors are their default
    /// values.
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Retrieves the set color for the given material, or the default if
    /// none is set.
    #[inline]
    pub fn get_color(&self, material: TerrainColorMaterial) -> Color3uint8 {
        if let Some(color) = self.inner.get(&material) {
            *color
        } else {
            material.default_color()
        }
    }

    /// Sets the color for the given material.
    #[inline]
    pub fn set_color(&mut self, material: TerrainColorMaterial, color: Color3uint8) {
        self.inner.insert(material, color);
    }

    /// Encodes the `MaterialColors` into a binary blob that can be understood
    /// by Roblox.
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(69);
        // 6 reserved bytes
        buffer.extend_from_slice(&[0; 6]);

        for color in MATERIAL_ORDER {
            let color = self.get_color(color);
            buffer.extend_from_slice(&[color.r, color.g, color.b])
        }

        buffer
    }

    /// Decodes a `MaterialColors` from a binary blob. The blob must be
    /// the same format used by `encode` and Roblox.
    pub fn decode(buffer: &[u8]) -> Result<Self, CrateError> {
        if buffer.len() != 69 {
            return Err(MaterialColorsError::WrongLength(buffer.len()).into());
        }
        let mut map = BTreeMap::new();
        // We have to skip the first 6 bytes, which amounts to 2 chunks
        for (material, color) in MATERIAL_ORDER.iter().zip(buffer.chunks(3).skip(2)) {
            map.insert(*material, Color3uint8::new(color[0], color[1], color[2]));
        }

        Ok(Self { inner: map })
    }
}

impl<T> From<T> for MaterialColors
where
    T: Into<BTreeMap<TerrainColorMaterial, Color3uint8>>,
{
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

/// An error that can occur when deserializing or working with MaterialColors and TerrainColorMaterial.
#[derive(Debug, Error)]
pub(crate) enum MaterialColorsError {
    /// The `MaterialColors` blob was the wrong number of bytes.
    #[error(
        "MaterialColors blob was the wrong length (expected it to be 69 bytes, it was {0} bytes)"
    )]
    WrongLength(usize),
    /// The argument provided to `from_str` did not correspond to a known
    /// TerrainMaterial.
    #[error("cannot convert `{0}` into TerrainMaterial")]
    UnknownMaterial(String),
}

/// Constructs an enum named `TerrainColorMaterial` for all values contained in
/// `MaterialColors` alongside a mapping for a default color for that material.
///
/// Additionally, makes a constant named `MATERIAL_ORDER` that indicates what
/// order the colors must be written and read in.
macro_rules! material_colors {
    ($($name:ident => [$r:literal, $g:literal, $b:literal]),*$(,)?) => {
        // A downside to the macro is that the length of `MATERIAL_ORDER`
        // is hardcoded. There are ways to count macro repetitions, but they
        // all have tangible downsides.
        // See: https://danielkeep.github.io/tlborm/book/blk-counting.html

        /// A list of all `TerrainColorMaterial` in the order they must be read
        /// and written.
        const MATERIAL_ORDER: [TerrainColorMaterial; 21] = [$(TerrainColorMaterial::$name,)*];

        /// All materials that are represented by `MaterialColors`.
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
        )]
        enum TerrainColorMaterial {
            $(
                $name,
            )*
        }

        impl TerrainColorMaterial {
            /// Returns the default color for the given `TerrainMaterial`.
            pub fn default_color(&self) -> Color3uint8 {
                match self {
                    $(
                        Self::$name => Color3uint8::new($r, $g, $b),
                    )*
                }
            }
        }

        impl FromStr for TerrainColorMaterial {
            type Err = CrateError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {$(
                    stringify!($name) => Ok(Self::$name),
                )*
                    _ => Err(MaterialColorsError::UnknownMaterial(s.to_string()).into()),
                }
            }
        }
    };
}

material_colors! {
    Grass => [106, 127, 63],
    Slate => [63, 127, 107],
    Concrete => [127, 102, 63],
    Brick => [138, 86, 62],
    Sand => [143, 126, 95],
    WoodPlanks => [139, 109, 79],
    Rock => [102, 108, 111],
    Glacier => [101, 176, 234],
    Snow => [195, 199, 218],
    Sandstone => [137, 90, 71],
    Mud => [58, 46, 36],
    Basalt => [30, 30, 37],
    Ground => [102, 92, 59],
    CrackedLava => [232, 156, 74],
    Asphalt => [115, 123, 107],
    Cobblestone => [132, 123, 90],
    Ice => [129, 194, 224],
    LeafyGrass => [115, 132, 74],
    Salt => [198, 189, 181],
    Limestone => [206, 173, 148],
    Pavement => [148, 148, 140],
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn deserialize() {
        let serialized = r#"{
            "Grass": [10, 20, 30],
            "Mud": [255, 0, 127]
        }"#;
        let expected: MaterialColors = serde_json::from_str(serialized).unwrap();

        assert_eq!(
            expected.get_color(TerrainColorMaterial::Grass),
            Color3uint8::new(10, 20, 30),
        );
        assert_eq!(
            expected.get_color(TerrainColorMaterial::Mud),
            Color3uint8::new(255, 0, 127),
        );

        assert_eq!(
            expected.get_color(TerrainColorMaterial::Brick),
            TerrainColorMaterial::Brick.default_color()
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialize() {
        let mut colors = MaterialColors::new();
        colors.set_color(TerrainColorMaterial::Grass, Color3uint8::new(10, 20, 30));
        colors.set_color(TerrainColorMaterial::Mud, Color3uint8::new(255, 0, 127));

        assert_eq!(
            serde_json::to_string(&colors).unwrap(),
            r#"{"Grass":[10,20,30],"Mud":[255,0,127]}"#
        )
    }

    #[test]
    fn decode_defaults() {
        // Default MaterialColors but base64
        let blob = base64::decode("AAAAAAAAan8/P39rf2Y/ilY+j35fi21PZmxvZbDqw8faiVpHOi4kHh4lZlw76JxKc3trhHtagcLgc4RKxr21zq2UlJSM").unwrap();
        let colors = MaterialColors::decode(&blob).unwrap();

        for color in MATERIAL_ORDER {
            assert_eq!(
                colors.get_color(color),
                color.default_color(),
                "{color:?} did not match"
            )
        }
    }

    #[test]
    fn decode_sequential() {
        use std::convert::TryFrom;

        // MaterialColors but every color is sequentially laid out
        // Grass = [1, 2, 3], Slate = [4, 5, 6], etc.
        let blob = base64::decode("AAAAAAAAAQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyAhIiMkJSYnKCkqKywtLi8wMTIzNDU2Nzg5Ojs8PT4/").unwrap();
        let colors = MaterialColors::decode(&blob).unwrap();

        for (n, color) in MATERIAL_ORDER.iter().enumerate() {
            let r = u8::try_from(n * 3 + 1).unwrap();
            let g = u8::try_from(n * 3 + 2).unwrap();
            let b = u8::try_from(n * 3 + 3).unwrap();
            assert_eq!(
                colors.get_color(*color),
                Color3uint8::new(r, g, b),
                "{color:?} did not match"
            );
        }
    }

    #[test]
    fn encode_defaults() {
        let colors = MaterialColors::new();
        let blob = base64::encode(colors.encode());

        assert_eq!(blob, "AAAAAAAAan8/P39rf2Y/ilY+j35fi21PZmxvZbDqw8faiVpHOi4kHh4lZlw76JxKc3trhHtagcLgc4RKxr21zq2UlJSM");
    }

    #[test]
    fn encode_sequential() {
        use std::convert::TryFrom;

        let mut colors = MaterialColors::new();

        for (n, color) in MATERIAL_ORDER.iter().enumerate() {
            let r = u8::try_from(n * 3 + 1).unwrap();
            let g = u8::try_from(n * 3 + 2).unwrap();
            let b = u8::try_from(n * 3 + 3).unwrap();

            colors.set_color(*color, Color3uint8::new(r, g, b))
        }
        let blob = base64::encode(colors.encode());

        assert_eq!(blob, "AAAAAAAAAQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyAhIiMkJSYnKCkqKywtLi8wMTIzNDU2Nzg5Ojs8PT4/");
    }

    #[test]
    fn from_str_materials() {
        assert!(TerrainColorMaterial::from_str("Grass").is_ok());
        assert!(TerrainColorMaterial::from_str("Concrete").is_ok());
        assert!(TerrainColorMaterial::from_str("Rock").is_ok());
        assert!(TerrainColorMaterial::from_str("Asphalt").is_ok());
        assert!(TerrainColorMaterial::from_str("Salt").is_ok());
        assert!(TerrainColorMaterial::from_str("Pavement").is_ok());

        assert!(
            TerrainColorMaterial::from_str("A name I am certain Roblox will never add").is_err()
        );
        // `from_str` is case-sensitive
        assert!(TerrainColorMaterial::from_str("gRaSs").is_err());
    }
}
