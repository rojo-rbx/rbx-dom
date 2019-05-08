#![allow(clippy::unreadable_literal)]
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

			pub fn as_rgb(self) -> [f32; 3] {
				match self {
					$(
						BrickColor::$enum => [$color3_r, $color3_g, $color3_b],
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
  [SlimeGreen, "Slime green", 1, (0.313726, 0.427451, 0.329412)],
  [BrightBluishGreen, "Bright bluish green", 2, (0.0, 0.560784, 0.611765)],
  [Black, "Black", 3, (0.105882, 0.164706, 0.207843)],
  [DeepBlue, "Deep blue", 4, (0.129412, 0.329412, 0.72549)],
  [DarkBlue, "Dark blue", 5, (0.0, 0.0627451, 0.690196)],
  [NavyBlue, "Navy blue", 6, (0.0, 0.12549, 0.376471)],
  [ParsleyGreen, "Parsley green", 7, (0.172549, 0.396078, 0.113725)],
  [DarkGreen, "Dark green", 8, (0.156863, 0.498039, 0.278431)],
  [Teal, "Teal", 9, (0.0705882, 0.933333, 0.831373)],
  [SmokyGrey, "Smoky grey", 10, (0.356863, 0.364706, 0.411765)],
  [SteelBlue, "Steel blue", 11, (0.321569, 0.486275, 0.682353)],
  [StormBlue, "Storm blue", 12, (0.2, 0.345098, 0.509804)],
  [Lapis, "Lapis", 13, (0.0627451, 0.164706, 0.862745)],
  [DarkIndigo, "Dark indigo", 14, (0.239216, 0.0823529, 0.521569)],
  [Camo, "Camo", 15, (0.227451, 0.490196, 0.0823529)],
  [SeaGreen, "Sea green", 16, (0.203922, 0.556863, 0.25098)],
  [Shamrock, "Shamrock", 17, (0.356863, 0.603922, 0.298039)],
  [Toothpaste, "Toothpaste", 18, (0.0, 1.0, 1.0)],
  [SandBlue, "Sand blue", 19, (0.454902, 0.52549, 0.615686)],
  [MediumBlue, "Medium blue", 20, (0.431373, 0.6, 0.792157)],
  [BrightBlue, "Bright blue", 21, (0.0509804, 0.411765, 0.67451)],
  [ReallyBlue, "Really blue", 22, (0.0, 0.0, 1.0)],
  [Mulberry, "Mulberry", 23, (0.34902, 0.133333, 0.34902)],
  [ForestGreen, "Forest green", 24, (0.121569, 0.501961, 0.113725)],
  [BrightGreen, "Bright green", 25, (0.294118, 0.592157, 0.294118)],
  [Grime, "Grime", 26, (0.498039, 0.556863, 0.392157)],
  [LimeGreen, "Lime green", 27, (0.0, 1.0, 0.0)],
  [PastelBlueGreen, "Pastel blue-green", 28, (0.623529, 0.952941, 0.913726)],
  [Fossil, "Fossil", 29, (0.623529, 0.631373, 0.67451)],
  [ElectricBlue, "Electric blue", 30, (0.0352941, 0.537255, 0.811765)],
  [Lavender, "Lavender", 31, (0.54902, 0.356863, 0.623529)],
  [RoyalPurple, "Royal purple", 32, (0.384314, 0.145098, 0.819608)],
  [Eggplant, "Eggplant", 33, (0.482353, 0.0, 0.482353)],
  [SandGreen, "Sand green", 34, (0.470588, 0.564706, 0.509804)],
  [Moss, "Moss", 35, (0.486275, 0.611765, 0.419608)],
  [Artichoke, "Artichoke", 36, (0.541176, 0.670588, 0.521569)],
  [SageGreen, "Sage green", 37, (0.72549, 0.768628, 0.694118)],
  [PastelLightBlue, "Pastel light blue", 38, (0.686275, 0.866667, 1.0)],
  [CadetBlue, "Cadet blue", 39, (0.623529, 0.678431, 0.752941)],
  [Cyan, "Cyan", 40, (0.0156863, 0.686275, 0.92549)],
  [Alder, "Alder", 41, (0.705882, 0.501961, 1.0)],
  [Lilac, "Lilac", 42, (0.654902, 0.368627, 0.607843)],
  [Plum, "Plum", 43, (0.482353, 0.184314, 0.482353)],
  [BrightViolet, "Bright violet", 44, (0.419608, 0.196078, 0.486275)],
  [Olive, "Olive", 45, (0.756863, 0.745098, 0.258824)],
  [BrightYellowishGreen, "Br. yellowish green", 46, (0.643137, 0.741176, 0.278431)],
  [Olivine, "Olivine", 47, (0.580392, 0.745098, 0.505882)],
  [LaurelGreen, "Laurel green", 48, (0.658824, 0.741176, 0.6)],
  [QuillGrey, "Quill grey", 49, (0.87451, 0.87451, 0.870588)],
  [GhostGrey, "Ghost grey", 50, (0.792157, 0.796079, 0.819608)],
  [PastelBlue, "Pastel Blue", 51, (0.501961, 0.733333, 0.858824)],
  [PastelViolet, "Pastel violet", 52, (0.694118, 0.654902, 1.0)],
  [Pink, "Pink", 53, (1.0, 0.4, 0.8)],
  [HotPink, "Hot pink", 54, (1.0, 0.0, 0.74902)],
  [Magenta, "Magenta", 55, (0.666667, 0.0, 0.666667)],
  [Crimson, "Crimson", 56, (0.592157, 0.0, 0.0)],
  [DeepOrange, "Deep orange", 57, (1.0, 0.690196, 0.0)],
  [NewYeller, "New Yeller", 58, (1.0, 1.0, 0.0)],
  [MediumGreen, "Medium green", 59, (0.631373, 0.768628, 0.54902)],
  [Mint, "Mint", 60, (0.694118, 0.898039, 0.65098)],
  [PastelGreen, "Pastel green", 61, (0.8, 1.0, 0.8)],
  [LightStoneGrey, "Light stone grey", 62, (0.898039, 0.894118, 0.87451)],
  [LightBlue, "Light blue", 63, (0.705882, 0.823529, 0.894118)],
  [BabyBlue, "Baby blue", 64, (0.596078, 0.760784, 0.858824)],
  [CarnationPink, "Carnation pink", 65, (1.0, 0.596078, 0.862745)],
  [Persimmon, "Persimmon", 66, (1.0, 0.34902, 0.34902)],
  [ReallyRed, "Really red", 67, (1.0, 0.0, 0.0)],
  [BrightRed, "Bright red", 68, (0.768628, 0.156863, 0.109804)],
  [Maroon, "Maroon", 69, (0.458824, 0.0, 0.0)],
  [Gold, "Gold", 70, (0.937255, 0.721569, 0.219608)],
  [BrightYellow, "Bright yellow", 71, (0.960784, 0.803922, 0.188235)],
  [DaisyOrange, "Daisy orange", 72, (0.972549, 0.85098, 0.427451)],
  [CoolYellow, "Cool yellow", 73, (0.992157, 0.917647, 0.552941)],
  [PastelYellow, "Pastel yellow", 74, (1.0, 1.0, 0.8)],
  [Pearl, "Pearl", 75, (0.905882, 0.905882, 0.92549)],
  [Fog, "Fog", 76, (0.780392, 0.831373, 0.894118)],
  [Mauve, "Mauve", 77, (0.878431, 0.698039, 0.815686)],
  [Sunrise, "Sunrise", 78, (0.831373, 0.564706, 0.741176)],
  [TerraCotta, "Terra Cotta", 79, (0.745098, 0.407843, 0.384314)],
  [DustyRose, "Dusty Rose", 80, (0.639216, 0.294118, 0.294118)],
  [Cocoa, "Cocoa", 81, (0.337255, 0.141176, 0.141176)],
  [NeonOrange, "Neon orange", 82, (0.835294, 0.45098, 0.239216)],
  [BrightOrange, "Bright orange", 83, (0.854902, 0.521569, 0.254902)],
  [Wheat, "Wheat", 84, (0.945098, 0.905882, 0.780392)],
  [Buttermilk, "Buttermilk", 85, (0.996078, 0.952941, 0.733333)],
  [InstitutionalWhite, "Institutional white", 86, (0.972549, 0.972549, 0.972549)],
  [White, "White", 87, (0.94902, 0.952941, 0.952941)],
  [LightReddishViolet, "Light reddish violet", 88, (0.909804, 0.729412, 0.784314)],
  [PastelOrange, "Pastel orange", 89, (1.0, 0.788235, 0.788235)],
  [Salmon, "Salmon", 90, (1.0, 0.580392, 0.580392)],
  [Tawny, "Tawny", 91, (0.588235, 0.333333, 0.333333)],
  [Rust, "Rust", 92, (0.560784, 0.298039, 0.164706)],
  [CgaBrown, "CGA brown", 93, (0.666667, 0.333333, 0.0)],
  [BrightYellowishOrange, "Br. yellowish orange", 94, (0.886275, 0.607843, 0.25098)],
  [Cashmere, "Cashmere", 95, (0.827451, 0.745098, 0.588235)],
  [Khaki, "Khaki", 96, (0.886275, 0.862745, 0.737255)],
  [LilyWhite, "Lily white", 97, (0.929412, 0.917647, 0.917647)],
  [Seashell, "Seashell", 98, (0.913726, 0.854902, 0.854902)],
  [PastelBrown, "Pastel brown", 99, (1.0, 0.8, 0.6)],
  [LightOrange, "Light orange", 100, (0.917647, 0.721569, 0.572549)],
  [MediumRed, "Medium red", 101, (0.854902, 0.52549, 0.478431)],
  [Burgundy, "Burgundy", 102, (0.533333, 0.243137, 0.243137)],
  [ReddishBrown, "Reddish brown", 103, (0.411765, 0.25098, 0.156863)],
  [Cork, "Cork", 104, (0.737255, 0.607843, 0.364706)],
  [Burlap, "Burlap", 105, (0.780392, 0.67451, 0.470588)],
  [Beige, "Beige", 106, (0.792157, 0.74902, 0.639216)],
  [Oyster, "Oyster", 107, (0.733333, 0.701961, 0.698039)],
  [MidGray, "Mid gray", 108, (0.803922, 0.803922, 0.803922)],
  [BrickYellow, "Brick yellow", 109, (0.843137, 0.772549, 0.603922)],
  [Nougat, "Nougat", 110, (0.8, 0.556863, 0.411765)],
  [Brown, "Brown", 111, (0.486275, 0.360784, 0.27451)],
  [PineCone, "Pine Cone", 112, (0.423529, 0.345098, 0.294118)],
  [FawnBrown, "Fawn brown", 113, (0.627451, 0.517647, 0.309804)],
  [SandRed, "Sand red", 114, (0.584314, 0.47451, 0.466667)],
  [HurricaneGrey, "Hurricane grey", 115, (0.584314, 0.537255, 0.533333)],
  [CloudyGrey, "Cloudy grey", 116, (0.670588, 0.658824, 0.619608)],
  [Linen, "Linen", 117, (0.686275, 0.580392, 0.513726)],
  [Copper, "Copper", 118, (0.588235, 0.403922, 0.4)],
  [DarkOrange, "Dark orange", 119, (0.627451, 0.372549, 0.207843)],
  [DirtBrown, "Dirt brown", 120, (0.337255, 0.258824, 0.211765)],
  [Bronze, "Bronze", 121, (0.494118, 0.407843, 0.247059)],
  [DarkStoneGrey, "Dark stone grey", 122, (0.388235, 0.372549, 0.384314)],
  [MediumStoneGrey, "Medium stone grey", 123, (0.639216, 0.635294, 0.647059)],
  [Flint, "Flint", 124, (0.411765, 0.4, 0.360784)],
  [DarkTaupe, "Dark taupe", 125, (0.352941, 0.298039, 0.258824)],
  [BurntSienna, "Burnt Sienna", 126, (0.415686, 0.223529, 0.0352941)],
  [ReallyBlack, "Really black", 127, (0.0666667, 0.0666667, 0.0666667)],
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
		assert_eq!(BrickColor::PastelBrown.as_rgb(), [1.0, 0.8, 0.6]);
	}
}
