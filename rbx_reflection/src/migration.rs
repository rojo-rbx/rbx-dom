use rbx_types::{Enum, Font, FontStyle, FontWeight, Variant};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(
        "Invalid type for migration {migration:?}: expected a type of {expected}, got {actual:?}"
    )]
    InvalidTypeForMigration {
        migration: MigrationOperation,
        expected: &'static str,
        actual: Variant,
    },
    #[error("Invalid value for migration {migration:?}: expected {expected}, got {actual:?}")]
    InvalidValueForMigration {
        migration: MigrationOperation,
        expected: &'static str,
        actual: Variant,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyMigration {
    #[serde(rename = "To")]
    pub new_property_name: String,
    migration: MigrationOperation,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MigrationOperation {
    IgnoreGuiInsetToScreenInsets,
    FontToFontFace,
    BrickColorToColor,
}

impl PropertyMigration {
    pub fn perform(&self, input: &Variant) -> Result<Variant, MigrationError> {
        match self.migration {
            MigrationOperation::IgnoreGuiInsetToScreenInsets => {
                if let Variant::Bool(value) = input {
                    if *value {
                        Ok(Enum::from_u32(1).into())
                    } else {
                        Ok(Enum::from_u32(2).into())
                    }
                } else {
                    Err(MigrationError::InvalidTypeForMigration {
                        migration: MigrationOperation::IgnoreGuiInsetToScreenInsets,
                        expected: "Enum",
                        actual: input.clone(),
                    })
                }
            }
            MigrationOperation::FontToFontFace => {
                if let Variant::Enum(value) = input {
                    let value = value.to_u32();
                    Ok(match value {
                        0 => Font::regular("rbxasset://fonts/families/LegacyArial.json"),
                        1 => Font::regular("rbxasset://fonts/families/Arial.json"),
                        2 => Font::new(
                            "rbxasset://fonts/families/Arial.json",
                            FontWeight::Bold,
                            FontStyle::Normal,
                        ),
                        3 => Font::regular("rbxasset://fonts/families/SourceSansPro.json"),
                        4 => Font::new(
                            "rbxasset://fonts/families/SourceSansPro.json",
                            FontWeight::Bold,
                            FontStyle::Normal,
                        ),
                        16 => Font::new(
                            "rbxasset://fonts/families/SourceSansPro.json",
                            FontWeight::SemiBold,
                            FontStyle::Normal,
                        ),
                        5 => Font::new(
                            "rbxasset://fonts/families/SourceSansPro.json",
                            FontWeight::Light,
                            FontStyle::Normal,
                        ),
                        6 => Font::new(
                            "rbxasset://fonts/families/SourceSansPro.json",
                            FontWeight::Regular,
                            FontStyle::Italic,
                        ),
                        7 => Font::regular("rbxasset://fonts/families/AccanthisADFStd.json"),
                        8 => Font::regular("rbxasset://fonts/families/Guru.json"),
                        9 => Font::regular("rbxasset://fonts/families/ComicNeueAngular.json"),
                        10 => Font::regular("rbxasset://fonts/families/Inconsolata.json"),
                        11 => Font::regular("rbxasset://fonts/families/HighwayGothic.json"),
                        12 => Font::regular("rbxasset://fonts/families/Zekton.json"),
                        13 => Font::regular("rbxasset://fonts/families/PressStart2P.json"),
                        14 => Font::regular("rbxasset://fonts/families/Balthazar.json"),
                        15 => Font::regular("rbxasset://fonts/families/RomanAntique.json"),
                        17 => Font::regular("rbxasset://fonts/families/GothamSSm.json"),
                        18 => Font::new(
                            "rbxasset://fonts/families/GothamSSm.json",
                            FontWeight::Medium,
                            FontStyle::Normal,
                        ),
                        19 => Font::new(
                            "rbxasset://fonts/families/GothamSSm.json",
                            FontWeight::Bold,
                            FontStyle::Normal,
                        ),
                        20 => Font::new(
                            "rbxasset://fonts/families/GothamSSm.json",
                            FontWeight::Heavy,
                            FontStyle::Normal,
                        ),
                        21 => Font::regular("rbxasset://fonts/families/AmaticSC.json"),
                        22 => Font::regular("rbxasset://fonts/families/Bangers.json"),
                        23 => Font::regular("rbxasset://fonts/families/Creepster.json"),
                        24 => Font::regular("rbxasset://fonts/families/DenkOne.json"),
                        25 => Font::regular("rbxasset://fonts/families/Fondamento.json"),
                        26 => Font::regular("rbxasset://fonts/families/FredokaOne.json"),
                        27 => Font::regular("rbxasset://fonts/families/GrenzeGotisch.json"),
                        28 => Font::regular("rbxasset://fonts/families/IndieFlower.json"),
                        29 => Font::regular("rbxasset://fonts/families/JosefinSans.json"),
                        30 => Font::regular("rbxasset://fonts/families/Jura.json"),
                        31 => Font::regular("rbxasset://fonts/families/Kalam.json"),
                        32 => Font::regular("rbxasset://fonts/families/LuckiestGuy.json"),
                        33 => Font::regular("rbxasset://fonts/families/Merriweather.json"),
                        34 => Font::regular("rbxasset://fonts/families/Michroma.json"),
                        35 => Font::regular("rbxasset://fonts/families/Nunito.json"),
                        36 => Font::regular("rbxasset://fonts/families/Oswald.json"),
                        37 => Font::regular("rbxasset://fonts/families/PatrickHand.json"),
                        38 => Font::regular("rbxasset://fonts/families/PermanentMarker.json"),
                        39 => Font::regular("rbxasset://fonts/families/Roboto.json"),
                        40 => Font::regular("rbxasset://fonts/families/RobotoCondensed.json"),
                        41 => Font::regular("rbxasset://fonts/families/RobotoMono.json"),
                        42 => Font::regular("rbxasset://fonts/families/Sarpanch.json"),
                        43 => Font::regular("rbxasset://fonts/families/SpecialElite.json"),
                        44 => Font::regular("rbxasset://fonts/families/TitilliumWeb.json"),
                        45 => Font::regular("rbxasset://fonts/families/Ubuntu.json"),
                        _ => {
                            return Err(MigrationError::InvalidValueForMigration {
                                migration: MigrationOperation::FontToFontFace,
                                expected: "a Font enum value between 0 and 45",
                                actual: input.clone(),
                            })
                        }
                    }
                    .into())
                } else {
                    Err(MigrationError::InvalidTypeForMigration {
                        migration: MigrationOperation::FontToFontFace,
                        expected: "Enum",
                        actual: input.clone(),
                    })
                }
            }
            MigrationOperation::BrickColorToColor => {
                if let Variant::BrickColor(color) = input {
                    Ok(color.to_color3uint8().into())
                } else {
                    Err(MigrationError::InvalidTypeForMigration {
                        migration: MigrationOperation::BrickColorToColor,
                        expected: "BrickColor",
                        actual: input.clone(),
                    })
                }
            }
        }
    }
}
