use serde_derive::{Serialize, Deserialize};
use std::fmt;

macro_rules! make_brick_color {
	({$([$enum:ident, $name:tt, $value:tt, ($color3_r:tt, $color3_g:tt, $color3_b:tt)],)+}) => {
		#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
		pub enum BrickColor {
			$($enum = $value,)+
		}

		impl BrickColor {
			pub fn from_palette(value: u8) -> Option<BrickColor> {
				match value {
					$(
						$value => Some(BrickColor::$enum),
					)+

					_ => None,
				}
			}

			pub(crate) fn as_rgb(self) -> [u8; 3] {
				match self {
					$(
						BrickColor::$enum => [$color3_r, $color3_g, $color3_b],
					)+
				}
			}

			pub(crate) fn as_rgb_f32(self) -> [f32; 3] {
				match self {
					$(
						BrickColor::$enum => [
							($color3_r as f32) / 255.0,
							($color3_g as f32) / 255.0,
							($color3_b as f32) / 255.0,
						],
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
	};
}

make_brick_color!({
	[SlimeGreen, "Slime green", 1, (80, 109, 84)],
	[BrightBluishGreen, "Bright bluish green", 2, (0, 143, 156)],
	[Black, "Black", 3, (27, 42, 53)],
	[DeepBlue, "Deep blue", 4, (33, 84, 185)],
	[DarkBlue, "Dark blue", 5, (0, 16, 176)],
	[NavyBlue, "Navy blue", 6, (0, 32, 96)],
	[ParsleyGreen, "Parsley green", 7, (44, 101, 29)],
	[DarkGreen, "Dark green", 8, (40, 127, 71)],
	[Teal, "Teal", 9, (18, 238, 212)],
	[SmokyGrey, "Smoky grey", 10, (91, 93, 105)],
	[SteelBlue, "Steel blue", 11, (82, 124, 174)],
	[StormBlue, "Storm blue", 12, (51, 88, 130)],
	[Lapis, "Lapis", 13, (16, 42, 220)],
	[DarkIndigo, "Dark indigo", 14, (61, 21, 133)],
	[Camo, "Camo", 15, (58, 125, 21)],
	[SeaGreen, "Sea green", 16, (52, 142, 64)],
	[Shamrock, "Shamrock", 17, (91, 154, 76)],
	[Toothpaste, "Toothpaste", 18, (0, 255, 255)],
	[SandBlue, "Sand blue", 19, (116, 134, 157)],
	[MediumBlue, "Medium blue", 20, (110, 153, 202)],
	[BrightBlue, "Bright blue", 21, (13, 105, 172)],
	[ReallyBlue, "Really blue", 22, (0, 0, 255)],
	[Mulberry, "Mulberry", 23, (89, 34, 89)],
	[ForestGreen, "Forest green", 24, (31, 128, 29)],
	[BrightGreen, "Bright green", 25, (75, 151, 75)],
	[Grime, "Grime", 26, (127, 142, 100)],
	[LimeGreen, "Lime green", 27, (0, 255, 0)],
	[PastelBlueGreen, "Pastel blue-green", 28, (159, 243, 233)],
	[Fossil, "Fossil", 29, (159, 161, 172)],
	[ElectricBlue, "Electric blue", 30, (9, 137, 207)],
	[Lavender, "Lavender", 31, (140, 91, 159)],
	[RoyalPurple, "Royal purple", 32, (98, 37, 209)],
	[Eggplant, "Eggplant", 33, (123, 0, 123)],
	[SandGreen, "Sand green", 34, (120, 144, 130)],
	[Moss, "Moss", 35, (124, 156, 107)],
	[Artichoke, "Artichoke", 36, (138, 171, 133)],
	[SageGreen, "Sage green", 37, (185, 196, 177)],
	[PastelLightBlue, "Pastel light blue", 38, (175, 221, 255)],
	[CadetBlue, "Cadet blue", 39, (159, 173, 192)],
	[Cyan, "Cyan", 40, (4, 175, 236)],
	[Alder, "Alder", 41, (180, 128, 255)],
	[Lilac, "Lilac", 42, (167, 94, 155)],
	[Plum, "Plum", 43, (123, 47, 123)],
	[BrightViolet, "Bright violet", 44, (107, 50, 124)],
	[Olive, "Olive", 45, (193, 190, 66)],
	[BrightYellowishGreen, "Br. yellowish green", 46, (164, 189, 71)],
	[Olivine, "Olivine", 47, (148, 190, 129)],
	[LaurelGreen, "Laurel green", 48, (168, 189, 153)],
	[QuillGrey, "Quill grey", 49, (223, 223, 222)],
	[GhostGrey, "Ghost grey", 50, (202, 203, 209)],
	[PastelBlue, "Pastel Blue", 51, (128, 187, 219)],
	[PastelViolet, "Pastel violet", 52, (177, 167, 255)],
	[Pink, "Pink", 53, (255, 102, 204)],
	[HotPink, "Hot pink", 54, (255, 0, 191)],
	[Magenta, "Magenta", 55, (170, 0, 170)],
	[Crimson, "Crimson", 56, (151, 0, 0)],
	[DeepOrange, "Deep orange", 57, (255, 176, 0)],
	[NewYeller, "New Yeller", 58, (255, 255, 0)],
	[MediumGreen, "Medium green", 59, (161, 196, 140)],
	[Mint, "Mint", 60, (177, 229, 166)],
	[PastelGreen, "Pastel green", 61, (204, 255, 204)],
	[LightStoneGrey, "Light stone grey", 62, (229, 228, 223)],
	[LightBlue, "Light blue", 63, (180, 210, 228)],
	[BabyBlue, "Baby blue", 64, (152, 194, 219)],
	[CarnationPink, "Carnation pink", 65, (255, 152, 220)],
	[Persimmon, "Persimmon", 66, (255, 89, 89)],
	[ReallyRed, "Really red", 67, (255, 0, 0)],
	[BrightRed, "Bright red", 68, (196, 40, 28)],
	[Maroon, "Maroon", 69, (117, 0, 0)],
	[Gold, "Gold", 70, (239, 184, 56)],
	[BrightYellow, "Bright yellow", 71, (245, 205, 48)],
	[DaisyOrange, "Daisy orange", 72, (248, 217, 109)],
	[CoolYellow, "Cool yellow", 73, (253, 234, 141)],
	[PastelYellow, "Pastel yellow", 74, (255, 255, 204)],
	[Pearl, "Pearl", 75, (231, 231, 236)],
	[Fog, "Fog", 76, (199, 212, 228)],
	[Mauve, "Mauve", 77, (224, 178, 208)],
	[Sunrise, "Sunrise", 78, (212, 144, 189)],
	[TerraCotta, "Terra Cotta", 79, (190, 104, 98)],
	[DustyRose, "Dusty Rose", 80, (163, 75, 75)],
	[Cocoa, "Cocoa", 81, (86, 36, 36)],
	[NeonOrange, "Neon orange", 82, (213, 115, 61)],
	[BrightOrange, "Bright orange", 83, (218, 133, 65)],
	[Wheat, "Wheat", 84, (241, 231, 199)],
	[Buttermilk, "Buttermilk", 85, (254, 243, 187)],
	[InstitutionalWhite, "Institutional white", 86, (248, 248, 248)],
	[White, "White", 87, (242, 243, 243)],
	[LightReddishViolet, "Light reddish violet", 88, (232, 186, 200)],
	[PastelOrange, "Pastel orange", 89, (255, 201, 201)],
	[Salmon, "Salmon", 90, (255, 148, 148)],
	[Tawny, "Tawny", 91, (150, 85, 85)],
	[Rust, "Rust", 92, (143, 76, 42)],
	[CgaBrown, "CGA brown", 93, (170, 85, 0)],
	[BrightYellowishOrange, "Br. yellowish orange", 94, (226, 155, 64)],
	[Cashmere, "Cashmere", 95, (211, 190, 150)],
	[Khaki, "Khaki", 96, (226, 220, 188)],
	[LilyWhite, "Lily white", 97, (237, 234, 234)],
	[Seashell, "Seashell", 98, (233, 218, 218)],
	[PastelBrown, "Pastel brown", 99, (255, 204, 153)],
	[LightOrange, "Light orange", 100, (234, 184, 146)],
	[MediumRed, "Medium red", 101, (218, 134, 122)],
	[Burgundy, "Burgundy", 102, (136, 62, 62)],
	[ReddishBrown, "Reddish brown", 103, (105, 64, 40)],
	[Cork, "Cork", 104, (188, 155, 93)],
	[Burlap, "Burlap", 105, (199, 172, 120)],
	[Beige, "Beige", 106, (202, 191, 163)],
	[Oyster, "Oyster", 107, (187, 179, 178)],
	[MidGray, "Mid gray", 108, (205, 205, 205)],
	[BrickYellow, "Brick yellow", 109, (215, 197, 154)],
	[Nougat, "Nougat", 110, (204, 142, 105)],
	[Brown, "Brown", 111, (124, 92, 70)],
	[PineCone, "Pine Cone", 112, (108, 88, 75)],
	[FawnBrown, "Fawn brown", 113, (160, 132, 79)],
	[SandRed, "Sand red", 114, (149, 121, 119)],
	[HurricaneGrey, "Hurricane grey", 115, (149, 137, 136)],
	[CloudyGrey, "Cloudy grey", 116, (171, 168, 158)],
	[Linen, "Linen", 117, (175, 148, 131)],
	[Copper, "Copper", 118, (150, 103, 102)],
	[DarkOrange, "Dark orange", 119, (160, 95, 53)],
	[DirtBrown, "Dirt brown", 120, (86, 66, 54)],
	[Bronze, "Bronze", 121, (126, 104, 63)],
	[DarkStoneGrey, "Dark stone grey", 122, (99, 95, 98)],
	[MediumStoneGrey, "Medium stone grey", 123, (163, 162, 165)],
	[Flint, "Flint", 124, (105, 102, 92)],
	[DarkTaupe, "Dark taupe", 125, (90, 76, 66)],
	[BurntSienna, "Burnt Sienna", 126, (106, 57, 9)],
	[ReallyBlack, "Really black", 127, (17, 17, 17)],
});

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn palette() {
		assert_eq!(BrickColor::from_palette(99).unwrap().to_string(), "Pastel brown");
	}

	#[test]
	fn as_rgb() {
		assert_eq!(BrickColor::PastelBrown.as_rgb(), [255, 204, 153]);
	}
}
