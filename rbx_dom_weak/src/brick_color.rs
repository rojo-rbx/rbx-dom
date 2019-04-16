use serde_derive::{Serialize, Deserialize};
use std::fmt;

macro_rules! make_brick_color {
	({$([$enum:ident, $name:tt, $value:tt],)+}) => {
		#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
		pub enum BrickColor {
			$($enum = $value,)+
		}

		impl BrickColor {
			pub fn from_palette(value: u8) -> Option<BrickColor> {
				match value.into() {
					$(
						$value => Some(BrickColor::$enum),
					)+

					_ => None,
				}
			}
		}

		impl fmt::Display for BrickColor {
			fn fmt(&self, writer: &mut fmt::Formatter) -> fmt::Result {
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
	[SlimeGreen, "Slime green", 1],
	[BrightBluishGreen, "Bright bluish green", 2],
	[Black, "Black", 3],
	[DeepBlue, "Deep blue", 4],
	[DarkBlue, "Dark blue", 5],
	[NavyBlue, "Navy blue", 6],
	[ParsleyGreen, "Parsley green", 7],
	[DarkGreen, "Dark green", 8],
	[Teal, "Teal", 9],
	[SmokyGrey, "Smoky grey", 10],
	[SteelBlue, "Steel blue", 11],
	[StormBlue, "Storm blue", 12],
	[Lapis, "Lapis", 13],
	[DarkIndigo, "Dark indigo", 14],
	[Camo, "Camo", 15],
	[SeaGreen, "Sea green", 16],
	[Shamrock, "Shamrock", 17],
	[Toothpaste, "Toothpaste", 18],
	[SandBlue, "Sand blue", 19],
	[MediumBlue, "Medium blue", 20],
	[BrightBlue, "Bright blue", 21],
	[ReallyBlue, "Really blue", 22],
	[Mulberry, "Mulberry", 23],
	[ForestGreen, "Forest green", 24],
	[BrightGreen, "Bright green", 25],
	[Grime, "Grime", 26],
	[LimeGreen, "Lime green", 27],
	[PastelBlueGreen, "Pastel blue-green", 28],
	[Fossil, "Fossil", 29],
	[ElectricBlue, "Electric blue", 30],
	[Lavender, "Lavender", 31],
	[RoyalPurple, "Royal purple", 32],
	[Eggplant, "Eggplant", 33],
	[SandGreen, "Sand green", 34],
	[Moss, "Moss", 35],
	[Artichoke, "Artichoke", 36],
	[SageGreen, "Sage green", 37],
	[PastelLightBlue, "Pastel light blue", 38],
	[CadetBlue, "Cadet blue", 39],
	[Cyan, "Cyan", 40],
	[Alder, "Alder", 41],
	[Lilac, "Lilac", 42],
	[Plum, "Plum", 43],
	[BrightViolet, "Bright violet", 44],
	[Olive, "Olive", 45],
	[BrightYellowishGreen, "Br. yellowish green", 46],
	[Olivine, "Olivine", 47],
	[LaurelGreen, "Laurel green", 48],
	[QuillGrey, "Quill grey", 49],
	[GhostGrey, "Ghost grey", 50],
	[PastelBlue, "Pastel Blue", 51],
	[PastelViolet, "Pastel violet", 52],
	[Pink, "Pink", 53],
	[HotPink, "Hot pink", 54],
	[Magenta, "Magenta", 55],
	[Crimson, "Crimson", 56],
	[DeepOrange, "Deep orange", 57],
	[NewYeller, "New Yeller", 58],
	[MediumGreen, "Medium green", 59],
	[Mint, "Mint", 60],
	[PastelGreen, "Pastel green", 61],
	[LightStoneGrey, "Light stone grey", 62],
	[LightBlue, "Light blue", 63],
	[BabyBlue, "Baby blue", 64],
	[CarnationPink, "Carnation pink", 65],
	[Persimmon, "Persimmon", 66],
	[ReallyRed, "Really red", 67],
	[BrightRed, "Bright red", 68],
	[Maroon, "Maroon", 69],
	[Gold, "Gold", 70],
	[BrightYellow, "Bright yellow", 71],
	[DaisyOrange, "Daisy orange", 72],
	[CoolYellow, "Cool yellow", 73],
	[PastelYellow, "Pastel yellow", 74],
	[Pearl, "Pearl", 75],
	[Fog, "Fog", 76],
	[Mauve, "Mauve", 77],
	[Sunrise, "Sunrise", 78],
	[TerraCotta, "Terra Cotta", 79],
	[DustyRose, "Dusty Rose", 80],
	[Cocoa, "Cocoa", 81],
	[NeonOrange, "Neon orange", 82],
	[BrightOrange, "Bright orange", 83],
	[Wheat, "Wheat", 84],
	[Buttermilk, "Buttermilk", 85],
	[InstitutionalWhite, "Institutional white", 86],
	[White, "White", 87],
	[LightReddishViolet, "Light reddish violet", 88],
	[PastelOrange, "Pastel orange", 89],
	[Salmon, "Salmon", 90],
	[Tawny, "Tawny", 91],
	[Rust, "Rust", 92],
	[CGABrown, "CGA brown", 93],
	[BrightYellowishOrange, "Br. yellowish orange", 94],
	[Cashmere, "Cashmere", 95],
	[Khaki, "Khaki", 96],
	[LilyWhite, "Lily white", 97],
	[Seashell, "Seashell", 98],
	[PastelBrown, "Pastel brown", 99],
	[LightOrange, "Light orange", 100],
	[MediumRed, "Medium red", 101],
	[Burgundy, "Burgundy", 102],
	[ReddishBrown, "Reddish brown", 103],
	[Cork, "Cork", 104],
	[Burlap, "Burlap", 105],
	[Beige, "Beige", 106],
	[Oyster, "Oyster", 107],
	[MidGray, "Mid gray", 108],
	[BrickYellow, "Brick yellow", 109],
	[Nougat, "Nougat", 110],
	[Brown, "Brown", 111],
	[PineCone, "Pine Cone", 112],
	[FawnBrown, "Fawn brown", 113],
	[SandRed, "Sand red", 114],
	[HurricaneGrey, "Hurricane grey", 115],
	[CloudyGrey, "Cloudy grey", 116],
	[Linen, "Linen", 117],
	[Copper, "Copper", 118],
	[DarkOrange, "Dark orange", 119],
	[DirtBrown, "Dirt brown", 120],
	[Bronze, "Bronze", 121],
	[DarkStoneGrey, "Dark stone grey", 122],
	[MediumStoneGrey, "Medium stone grey", 123],
	[Flint, "Flint", 124],
	[DarkTaupe, "Dark taupe", 125],
	[BurntSienna, "Burnt Sienna", 126],
	[ReallyBlack, "Really black", 127],
});
