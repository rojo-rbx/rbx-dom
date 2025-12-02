use std::str::FromStr;

use thiserror::Error;

use crate::Color3uint8;

use crate::Error as CrateError;

#[derive(Debug, PartialEq, Clone)]
/// Represents the mapping of materials to colors used by Roblox's `Terrain`.
pub struct MaterialColors {
    /// The underlying map used by this struct. A `BTreeMap` is used
    /// over a `HashMap` to ensure serialization with serde is ordered.
    inner: [Color3uint8; NUM_COLORS],
}

impl MaterialColors {
    /// Constructs a new `MaterialColors` where all colors are their default
    /// values.
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: DEFAULT_COLORS,
        }
    }

    /// Retrieves the set color for the given material, or the default if
    /// none is set.
    #[inline]
    pub fn get_color(&self, material: TerrainMaterials) -> Color3uint8 {
        self.inner[material as usize]
    }

    /// Sets the color for the given material.
    #[inline]
    pub fn set_color(&mut self, material: TerrainMaterials, color: Color3uint8) {
        self.inner[material as usize] = color;
    }

    /// Encodes the `MaterialColors` into a binary blob that can be understood
    /// by Roblox.
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(69);
        // 6 reserved bytes
        buffer.extend_from_slice(&[0; 6]);

        for (_, color) in self {
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
        let colors = buffer
            .chunks(3)
            // We have to skip the first 6 bytes, which amounts to 2 chunks
            .skip(2)
            .map(|color| Color3uint8::new(color[0], color[1], color[2]));

        Ok(IntoIterator::into_iter(MATERIAL_ORDER)
            .zip(colors)
            .collect())
    }
}

impl Default for MaterialColors {
    fn default() -> Self {
        Self::new()
    }
}

impl core::iter::FromIterator<(TerrainMaterials, Color3uint8)> for MaterialColors {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (TerrainMaterials, Color3uint8)>,
    {
        let mut material_colors = Self::new();
        for (material, color) in iter {
            material_colors.set_color(material, color);
        }
        material_colors
    }
}

impl IntoIterator for MaterialColors {
    type Item = (TerrainMaterials, Color3uint8);
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: IntoIterator::into_iter(self.inner).enumerate(),
        }
    }
}
impl IntoIterator for &MaterialColors {
    type Item = (TerrainMaterials, Color3uint8);
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: IntoIterator::into_iter(self.inner).enumerate(),
        }
    }
}

pub struct Iter {
    inner: core::iter::Enumerate<core::array::IntoIter<Color3uint8, 21>>,
}
impl Iterator for Iter {
    type Item = (TerrainMaterials, Color3uint8);
    fn next(&mut self) -> Option<Self::Item> {
        let (i, color) = self.inner.next()?;
        Some((MATERIAL_ORDER[i], color))
    }
}

/// An error that can occur when deserializing or working with MaterialColors and TerrainMaterials.
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

/// Constructs an enum named `TerrainMaterials` for all values contained in
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

        /// A list of all `TerrainMaterials` in the order they must be read
        /// and written.
        const MATERIAL_ORDER: [TerrainMaterials; NUM_COLORS] = [$(TerrainMaterials::$name,)*];
        const DEFAULT_COLORS: [Color3uint8; NUM_COLORS] = [$(TerrainMaterials::$name.default_color(),)*];

        /// All materials that are represented by `MaterialColors`.
        ///
        /// Roblox may add more to this enum unexpectedly, so it is marked
        /// as `non_exhaustive.`
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
        )]
        #[non_exhaustive]
        pub enum TerrainMaterials {
            $(
                $name,
            )*
        }

        impl TerrainMaterials {
            /// Returns the default color for the given `TerrainMaterial`.
            pub const fn default_color(&self) -> Color3uint8 {
                match self {
                    $(
                        Self::$name => Color3uint8::new($r, $g, $b),
                    )*
                }
            }
        }

        impl FromStr for TerrainMaterials {
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

// If this goes above 21, Variant will become larger than 64 bytes.
const NUM_COLORS: usize = 21;
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

#[cfg(feature = "serde")]
impl serde::Serialize for MaterialColors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;
        for (material, color) in self {
            if color != material.default_color() {
                map.serialize_entry(&material, &color)?;
            }
        }
        map.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MaterialColors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct MaterialColorsVisitor;
        impl<'de> Visitor<'de> for MaterialColorsVisitor {
            type Value = MaterialColors;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a MaterialColors value")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut material_colors = MaterialColors::new();
                while let Some((material, color)) = map.next_entry()? {
                    material_colors.set_color(material, color);
                }
                Ok(material_colors)
            }
        }

        deserializer.deserialize_map(MaterialColorsVisitor)
    }
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
            expected.get_color(TerrainMaterials::Grass),
            Color3uint8::new(10, 20, 30),
        );
        assert_eq!(
            expected.get_color(TerrainMaterials::Mud),
            Color3uint8::new(255, 0, 127),
        );

        assert_eq!(
            expected.get_color(TerrainMaterials::Brick),
            TerrainMaterials::Brick.default_color()
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialize() {
        let mut colors = MaterialColors::new();
        colors.set_color(TerrainMaterials::Grass, Color3uint8::new(10, 20, 30));
        colors.set_color(TerrainMaterials::Mud, Color3uint8::new(255, 0, 127));

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
        assert!(TerrainMaterials::from_str("Grass").is_ok());
        assert!(TerrainMaterials::from_str("Concrete").is_ok());
        assert!(TerrainMaterials::from_str("Rock").is_ok());
        assert!(TerrainMaterials::from_str("Asphalt").is_ok());
        assert!(TerrainMaterials::from_str("Salt").is_ok());
        assert!(TerrainMaterials::from_str("Pavement").is_ok());

        assert!(TerrainMaterials::from_str("A name I am certain Roblox will never add").is_err());
        // `from_str` is case-sensitive
        assert!(TerrainMaterials::from_str("gRaSs").is_err());
    }
}
