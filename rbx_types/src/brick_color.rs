use std::fmt;

use crate::Color3uint8;

macro_rules! make_brick_color {
    ({
        $([
            $enum: ident,
            $name: expr,
            $value: expr,
            ($color3_r: expr, $color3_g: expr, $color3_b: expr)
        ],)+
    }) => {
        /// BrickColor values were the old, palette-based system of defining
        /// colors in Roblox. As of the time of writing, they're still used for
        /// some old systems like SpawnLocation and Team objects.
        ///
        /// Parts no longer use BrickColor, but we have conversions here to
        /// support older models.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u16)]
        #[non_exhaustive]
        pub enum BrickColor {
            $($enum = $value,)+
        }

        impl BrickColor {
            /// Find the first BrickColor with the given name, if it exists.
            ///
            /// Note that some colors (Lilac, Rust, Gold, and Deep orange) have
            /// name collisions and can only have one of their variants
            /// constructed from this function.
            ///
            /// This is roughly equivalent to `BrickColor.new(string)` from
            /// within Roblox, except unknown values will yield `None` instead
            /// of `Medium stone grey`.
            pub fn from_name(name: &str) -> Option<BrickColor> {
                // There are collisions in names of some colors. This should
                // work the same way that Roblox works when mapping names to
                // BrickColors!
                #[allow(unreachable_patterns)]
                match name {
                    $(
                        $name => Some(BrickColor::$enum),
                    )+

                    _ => None,
                }
            }

            /// Finds the BrickColor from its associated value. This is
            /// different from a BrickColor's _palette_ number, which not all
            /// colors have.
            ///
            /// This is roughly equivalent to `BrickColor.new(number)` from
            /// within Roblox, except unknown values will yield `None` instead
            /// of `Medium stone grey`.
            pub fn from_number(value: u16) -> Option<BrickColor> {
                match value {
                    $(
                        $value => Some(BrickColor::$enum),
                    )+

                    _ => None,
                }
            }

            pub fn to_color3uint8 (&self) -> Color3uint8 {
                match self {
                    $(
                        BrickColor::$enum => Color3uint8::new($color3_r, $color3_g, $color3_b),
                    )+
                }
            }
        }

        impl fmt::Display for BrickColor {
            fn fmt(&self, writer: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        BrickColor::$enum => write!(writer, $name),
                    )+
                }
            }
        }

        #[cfg(feature = "serde")]
        mod serde_impl {
            use super::*;

            use serde::{
                de::Error,
                Deserialize, Deserializer, Serialize, Serializer,
            };

            impl Serialize for BrickColor {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_u16(*self as u16)
                }
            }

            impl<'de> Deserialize<'de> for BrickColor {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let value = u16::deserialize(deserializer)?;

                    BrickColor::from_number(value).ok_or_else(|| {
                        D::Error::custom(format!("{} is not a valid BrickColor number", value))
                    })
                }
            }
        }
    };
}

make_brick_color!({
    [White, "White", 1, (242, 243, 243)],
    [Grey, "Grey", 2, (161, 165, 162)],
    [LightYellow, "Light yellow", 3, (249, 233, 153)],
    [BrickYellow, "Brick yellow", 5, (215, 197, 154)],
    [LightGreenMint, "Light green (Mint)", 6, (194, 218, 184)],
    [LightReddishViolet, "Light reddish violet", 9, (232, 186, 200)],
    [PastelBlue, "Pastel Blue", 11, (128, 187, 219)],
    [LightOrangeBrown, "Light orange brown", 12, (203, 132, 66)],
    [Nougat, "Nougat", 18, (204, 142, 105)],
    [BrightRed, "Bright red", 21, (196, 40, 28)],
    [MedReddishViolet, "Med. reddish violet", 22, (196, 112, 160)],
    [BrightBlue, "Bright blue", 23, (13, 105, 172)],
    [BrightYellow, "Bright yellow", 24, (245, 205, 48)],
    [EarthOrange, "Earth orange", 25, (98, 71, 50)],
    [Black, "Black", 26, (27, 42, 53)],
    [DarkGrey, "Dark grey", 27, (109, 110, 108)],
    [DarkGreen, "Dark green", 28, (40, 127, 71)],
    [MediumGreen, "Medium green", 29, (161, 196, 140)],
    [LigYellowichOrange, "Lig. Yellowich orange", 36, (243, 207, 155)],
    [BrightGreen, "Bright green", 37, (75, 151, 75)],
    [DarkOrange, "Dark orange", 38, (160, 95, 53)],
    [LightBluishViolet, "Light bluish violet", 39, (193, 202, 222)],
    [Transparent, "Transparent", 40, (236, 236, 236)],
    [TrRed, "Tr. Red", 41, (205, 84, 75)],
    [TrLgBlue, "Tr. Lg blue", 42, (193, 223, 240)],
    [TrBlue, "Tr. Blue", 43, (123, 182, 232)],
    [TrYellow, "Tr. Yellow", 44, (247, 241, 141)],
    [LightBlue, "Light blue", 45, (180, 210, 228)],
    [TrFluReddishOrange, "Tr. Flu. Reddish orange", 47, (217, 133, 108)],
    [TrGreen, "Tr. Green", 48, (132, 182, 141)],
    [TrFluGreen, "Tr. Flu. Green", 49, (248, 241, 132)],
    [PhosphWhite, "Phosph. White", 50, (236, 232, 222)],
    [LightRed, "Light red", 100, (238, 196, 182)],
    [MediumRed, "Medium red", 101, (218, 134, 122)],
    [MediumBlue, "Medium blue", 102, (110, 153, 202)],
    [LightGrey, "Light grey", 103, (199, 193, 183)],
    [BrightViolet, "Bright violet", 104, (107, 50, 124)],
    [BrYellowishOrange, "Br. yellowish orange", 105, (226, 155, 64)],
    [BrightOrange, "Bright orange", 106, (218, 133, 65)],
    [BrightBluishGreen, "Bright bluish green", 107, (0, 143, 156)],
    [EarthYellow, "Earth yellow", 108, (104, 92, 67)],
    [BrightBluishViolet, "Bright bluish violet", 110, (67, 84, 147)],
    [TrBrown, "Tr. Brown", 111, (191, 183, 177)],
    [MediumBluishViolet, "Medium bluish violet", 112, (104, 116, 172)],
    [TrMediReddishViolet, "Tr. Medi. reddish violet", 113, (229, 173, 200)],
    [MedYellowishGreen, "Med. yellowish green", 115, (199, 210, 60)],
    [MedBluishGreen, "Med. bluish green", 116, (85, 165, 175)],
    [LightBluishGreen, "Light bluish green", 118, (183, 215, 213)],
    [BrYellowishGreen, "Br. yellowish green", 119, (164, 189, 71)],
    [LigYellowishGreen, "Lig. yellowish green", 120, (217, 228, 167)],
    [MedYellowishOrange, "Med. yellowish orange", 121, (231, 172, 88)],
    [BrReddishOrange, "Br. reddish orange", 123, (211, 111, 76)],
    [BrightReddishViolet, "Bright reddish violet", 124, (146, 57, 120)],
    [LightOrange, "Light orange", 125, (234, 184, 146)],
    [TrBrightBluishViolet, "Tr. Bright bluish violet", 126, (165, 165, 203)],
    [Gold, "Gold", 127, (220, 188, 129)],
    [DarkNougat, "Dark nougat", 128, (174, 122, 89)],
    [Silver, "Silver", 131, (156, 163, 168)],
    [NeonOrange, "Neon orange", 133, (213, 115, 61)],
    [NeonGreen, "Neon green", 134, (216, 221, 86)],
    [SandBlue, "Sand blue", 135, (116, 134, 157)],
    [SandViolet, "Sand violet", 136, (135, 124, 144)],
    [MediumOrange, "Medium orange", 137, (224, 152, 100)],
    [SandYellow, "Sand yellow", 138, (149, 138, 115)],
    [EarthBlue, "Earth blue", 140, (32, 58, 86)],
    [EarthGreen, "Earth green", 141, (39, 70, 45)],
    [TrFluBlue, "Tr. Flu. Blue", 143, (207, 226, 247)],
    [SandBlueMetallic, "Sand blue metallic", 145, (121, 136, 161)],
    [SandVioletMetallic, "Sand violet metallic", 146, (149, 142, 163)],
    [SandYellowMetallic, "Sand yellow metallic", 147, (147, 135, 103)],
    [DarkGreyMetallic, "Dark grey metallic", 148, (87, 88, 87)],
    [BlackMetallic, "Black metallic", 149, (22, 29, 50)],
    [LightGreyMetallic, "Light grey metallic", 150, (171, 173, 172)],
    [SandGreen, "Sand green", 151, (120, 144, 130)],
    [SandRed, "Sand red", 153, (149, 121, 119)],
    [DarkRed, "Dark red", 154, (123, 46, 47)],
    [TrFluYellow, "Tr. Flu. Yellow", 157, (255, 246, 123)],
    [TrFluRed, "Tr. Flu. Red", 158, (225, 164, 194)],
    [GunMetallic, "Gun metallic", 168, (117, 108, 98)],
    [RedFlipFlop, "Red flip/flop", 176, (151, 105, 91)],
    [YellowFlipFlop, "Yellow flip/flop", 178, (180, 132, 85)],
    [SilverFlipFlop, "Silver flip/flop", 179, (137, 135, 136)],
    [Curry, "Curry", 180, (215, 169, 75)],
    [FireYellow, "Fire Yellow", 190, (249, 214, 46)],
    [FlameYellowishOrange, "Flame yellowish orange", 191, (232, 171, 45)],
    [ReddishBrown, "Reddish brown", 192, (105, 64, 40)],
    [FlameReddishOrange, "Flame reddish orange", 193, (207, 96, 36)],
    [MediumStoneGrey, "Medium stone grey", 194, (163, 162, 165)],
    [RoyalBlue, "Royal blue", 195, (70, 103, 164)],
    [DarkRoyalBlue, "Dark Royal blue", 196, (35, 71, 139)],
    [BrightReddishLilac, "Bright reddish lilac", 198, (142, 66, 133)],
    [DarkStoneGrey, "Dark stone grey", 199, (99, 95, 98)],
    [LemonMetalic, "Lemon metalic", 200, (130, 138, 93)],
    [LightStoneGrey, "Light stone grey", 208, (229, 228, 223)],
    [DarkCurry, "Dark Curry", 209, (176, 142, 68)],
    [FadedGreen, "Faded green", 210, (112, 149, 120)],
    [Turquoise, "Turquoise", 211, (121, 181, 181)],
    [LightRoyalBlue, "Light Royal blue", 212, (159, 195, 233)],
    [MediumRoyalBlue, "Medium Royal blue", 213, (108, 129, 183)],
    [Rust, "Rust", 216, (144, 76, 42)],
    [Brown, "Brown", 217, (124, 92, 70)],
    [ReddishLilac, "Reddish lilac", 218, (150, 112, 159)],
    [Lilac2, "Lilac", 219, (107, 98, 155)],
    [LightLilac, "Light lilac", 220, (167, 169, 206)],
    [BrightPurple, "Bright purple", 221, (205, 98, 152)],
    [LightPurple, "Light purple", 222, (228, 173, 200)],
    [LightPink, "Light pink", 223, (220, 144, 149)],
    [LightBrickYellow, "Light brick yellow", 224, (240, 213, 160)],
    [WarmYellowishOrange, "Warm yellowish orange", 225, (235, 184, 127)],
    [CoolYellow, "Cool yellow", 226, (253, 234, 141)],
    [DoveBlue, "Dove blue", 232, (125, 187, 221)],
    [MediumLilac, "Medium lilac", 268, (52, 43, 117)],
    [SlimeGreen, "Slime green", 301, (80, 109, 84)],
    [SmokyGrey, "Smoky grey", 302, (91, 93, 105)],
    [DarkBlue, "Dark blue", 303, (0, 16, 176)],
    [ParsleyGreen, "Parsley green", 304, (44, 101, 29)],
    [SteelBlue, "Steel blue", 305, (82, 124, 174)],
    [StormBlue, "Storm blue", 306, (51, 88, 130)],
    [Lapis, "Lapis", 307, (16, 42, 220)],
    [DarkIndigo, "Dark indigo", 308, (61, 21, 133)],
    [SeaGreen, "Sea green", 309, (52, 142, 64)],
    [Shamrock, "Shamrock", 310, (91, 154, 76)],
    [Fossil, "Fossil", 311, (159, 161, 172)],
    [Mulberry, "Mulberry", 312, (89, 34, 89)],
    [ForestGreen, "Forest green", 313, (31, 128, 29)],
    [CadetBlue, "Cadet blue", 314, (159, 173, 192)],
    [ElectricBlue, "Electric blue", 315, (9, 137, 207)],
    [Eggplant, "Eggplant", 316, (123, 0, 123)],
    [Moss, "Moss", 317, (124, 156, 107)],
    [Artichoke, "Artichoke", 318, (138, 171, 133)],
    [SageGreen, "Sage green", 319, (185, 196, 177)],
    [GhostGrey, "Ghost grey", 320, (202, 203, 209)],
    [Lilac, "Lilac", 321, (167, 94, 155)],
    [Plum, "Plum", 322, (123, 47, 123)],
    [Olivine, "Olivine", 323, (148, 190, 129)],
    [LaurelGreen, "Laurel green", 324, (168, 189, 153)],
    [QuillGrey, "Quill grey", 325, (223, 223, 222)],
    [Crimson, "Crimson", 327, (151, 0, 0)],
    [Mint, "Mint", 328, (177, 229, 166)],
    [BabyBlue, "Baby blue", 329, (152, 194, 219)],
    [CarnationPink, "Carnation pink", 330, (255, 152, 220)],
    [Persimmon, "Persimmon", 331, (255, 89, 89)],
    [Maroon, "Maroon", 332, (117, 0, 0)],
    [Gold2, "Gold", 333, (239, 184, 56)],
    [DaisyOrange, "Daisy orange", 334, (248, 217, 109)],
    [Pearl, "Pearl", 335, (231, 231, 236)],
    [Fog, "Fog", 336, (199, 212, 228)],
    [Salmon, "Salmon", 337, (255, 148, 148)],
    [TerraCotta, "Terra Cotta", 338, (190, 104, 98)],
    [Cocoa, "Cocoa", 339, (86, 36, 36)],
    [Wheat, "Wheat", 340, (241, 231, 199)],
    [Buttermilk, "Buttermilk", 341, (254, 243, 187)],
    [Mauve, "Mauve", 342, (224, 178, 208)],
    [Sunrise, "Sunrise", 343, (212, 144, 189)],
    [Tawny, "Tawny", 344, (150, 85, 85)],
    [Rust2, "Rust", 345, (143, 76, 42)],
    [Cashmere, "Cashmere", 346, (211, 190, 150)],
    [Khaki, "Khaki", 347, (226, 220, 188)],
    [LilyWhite, "Lily white", 348, (237, 234, 234)],
    [Seashell, "Seashell", 349, (233, 218, 218)],
    [Burgundy, "Burgundy", 350, (136, 62, 62)],
    [Cork, "Cork", 351, (188, 155, 93)],
    [Burlap, "Burlap", 352, (199, 172, 120)],
    [Beige, "Beige", 353, (202, 191, 163)],
    [Oyster, "Oyster", 354, (187, 179, 178)],
    [PineCone, "Pine Cone", 355, (108, 88, 75)],
    [FawnBrown, "Fawn brown", 356, (160, 132, 79)],
    [HurricaneGrey, "Hurricane grey", 357, (149, 137, 136)],
    [CloudyGrey, "Cloudy grey", 358, (171, 168, 158)],
    [Linen, "Linen", 359, (175, 148, 131)],
    [Copper, "Copper", 360, (150, 103, 102)],
    [DirtBrown, "Dirt brown", 361, (86, 66, 54)],
    [Bronze, "Bronze", 362, (126, 104, 63)],
    [Flint, "Flint", 363, (105, 102, 92)],
    [DarkTaupe, "Dark taupe", 364, (90, 76, 66)],
    [BurntSienna, "Burnt Sienna", 365, (106, 57, 9)],
    [InstitutionalWhite, "Institutional white", 1001, (248, 248, 248)],
    [MidGray, "Mid gray", 1002, (205, 205, 205)],
    [ReallyBlack, "Really black", 1003, (17, 17, 17)],
    [ReallyRed, "Really red", 1004, (255, 0, 0)],
    [DeepOrange, "Deep orange", 1005, (255, 176, 0)],
    [Alder, "Alder", 1006, (180, 128, 255)],
    [DustyRose, "Dusty Rose", 1007, (163, 75, 75)],
    [Olive, "Olive", 1008, (193, 190, 66)],
    [NewYeller, "New Yeller", 1009, (255, 255, 0)],
    [ReallyBlue, "Really blue", 1010, (0, 0, 255)],
    [NavyBlue, "Navy blue", 1011, (0, 32, 96)],
    [DeepBlue, "Deep blue", 1012, (33, 84, 185)],
    [Cyan, "Cyan", 1013, (4, 175, 236)],
    [CGABrown, "CGA brown", 1014, (170, 85, 0)],
    [Magenta, "Magenta", 1015, (170, 0, 170)],
    [Pink, "Pink", 1016, (255, 102, 204)],
    [DeepOrange2, "Deep orange", 1017, (255, 175, 0)],
    [Teal, "Teal", 1018, (18, 238, 212)],
    [Toothpaste, "Toothpaste", 1019, (0, 255, 255)],
    [LimeGreen, "Lime green", 1020, (0, 255, 0)],
    [Camo, "Camo", 1021, (58, 125, 21)],
    [Grime, "Grime", 1022, (127, 142, 100)],
    [Lavender, "Lavender", 1023, (140, 91, 159)],
    [PastelLightBlue, "Pastel light blue", 1024, (175, 221, 255)],
    [PastelOrange, "Pastel orange", 1025, (255, 201, 201)],
    [PastelViolet, "Pastel violet", 1026, (177, 167, 255)],
    [PastelBlueGreen, "Pastel blue-green", 1027, (159, 243, 233)],
    [PastelGreen, "Pastel green", 1028, (204, 255, 204)],
    [PastelYellow, "Pastel yellow", 1029, (255, 255, 204)],
    [PastelBrown, "Pastel brown", 1030, (255, 204, 153)],
    [RoyalPurple, "Royal purple", 1031, (98, 37, 209)],
    [HotPink, "Hot pink", 1032, (255, 0, 191)],
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_name() {
        assert_eq!(
            BrickColor::from_name("Pastel brown"),
            Some(BrickColor::PastelBrown)
        );
    }

    #[test]
    fn from_number() {
        assert_eq!(BrickColor::from_number(1030), Some(BrickColor::PastelBrown));
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human_ser() {
        let value = BrickColor::Gold2;
        let ser = serde_json::to_string(&value).unwrap();

        assert_eq!(ser, "333");
    }

    #[test]
    fn human_de() {
        let value: BrickColor = serde_json::from_str("1021").unwrap();
        assert_eq!(value, BrickColor::Camo);
    }

    #[test]
    fn non_human() {
        let value = BrickColor::Cork;

        let ser = bincode::serialize(&value).unwrap();
        let de: BrickColor = bincode::deserialize(&ser).unwrap();

        assert_eq!(de, value);
    }
}
