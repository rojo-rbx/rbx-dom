use rbx_types::{Content, Enum, Font, FontStyle, FontWeight, Variant};
use serde::{de, Deserialize, Deserializer, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum PropertyMigrationTarget<'a> {
    One(&'a str),
    Many(Vec<&'a str>),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyMigration<'a> {
    #[serde(rename = "To")]
    new_property_names: PropertyMigrationTarget<'a>,
    migration: MigrationOperation,
}

impl<'a, 'de: 'a> Deserialize<'de> for PropertyMigration<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct PropertyMigrationDeserialize<'a> {
            #[serde(borrow)]
            #[serde(rename = "To")]
            new_property_names: PropertyMigrationTarget<'a>,
            migration: MigrationOperation,
        }

        let migration = PropertyMigrationDeserialize::deserialize(deserializer)?;

        if let PropertyMigrationTarget::Many(names) = &migration.new_property_names {
            if names.is_empty() {
                return Err(de::Error::custom(
                    "property migration target list cannot be empty",
                ));
            }
        }

        Ok(Self {
            new_property_names: migration.new_property_names,
            migration: migration.migration,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum MigrationOperation {
    IgnoreGuiInsetToScreenInsets,
    FontToFontFace,
    BrickColorToColor,
    ContentIdToContent,
    CornerRadiusToCornerRadii,
    Int64ToContent,
}

impl PropertyMigration<'_> {
    pub fn new_property_names(&self) -> &[&str] {
        match &self.new_property_names {
            PropertyMigrationTarget::One(string) => std::slice::from_ref(string),
            PropertyMigrationTarget::Many(strings) => strings.as_slice(),
        }
    }

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
            MigrationOperation::ContentIdToContent => {
                if let Variant::ContentId(uri) = input {
                    let uri = uri.as_str();
                    if uri.is_empty() {
                        Ok(Content::none().into())
                    } else {
                        Ok(Content::from_uri(uri).into())
                    }
                } else {
                    Err(MigrationError::InvalidTypeForMigration {
                        migration: MigrationOperation::ContentIdToContent,
                        expected: "ContentId",
                        actual: input.clone(),
                    })
                }
            }
            MigrationOperation::CornerRadiusToCornerRadii => {
                if let Variant::UDim(_) = input {
                    Ok(input.clone())
                } else {
                    Err(MigrationError::InvalidTypeForMigration {
                        migration: MigrationOperation::CornerRadiusToCornerRadii,
                        expected: "UDim",
                        actual: input.clone(),
                    })
                }
            }
            MigrationOperation::Int64ToContent => {
                if let Variant::Int64(id) = input {
                    if *id <= 0 {
                        Ok(Content::none().into())
                    } else {
                        Ok(Content::from_uri(format!("rbxassetid://{id}")).into())
                    }
                } else {
                    Err(MigrationError::InvalidTypeForMigration {
                        migration: MigrationOperation::Int64ToContent,
                        expected: "Int64",
                        actual: input.clone(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_empty_property_target_list() {
        let error = serde_json::from_str::<PropertyMigration>(
            r#"{"To":[],"Migration":"CornerRadiusToCornerRadii"}"#,
        )
        .expect_err("empty target lists should fail to deserialize");

        assert!(error
            .to_string()
            .contains("property migration target list cannot be empty"));
    }

    #[test]
    fn deserialize_accepts_property_target_list_with_values() {
        let migration = serde_json::from_str::<PropertyMigration>(
            r#"{"To":["BottomLeftRadius","BottomRightRadius"],"Migration":"CornerRadiusToCornerRadii"}"#,
        )
        .expect("non-empty target lists should deserialize");

        assert_eq!(
            migration.new_property_names(),
            ["BottomLeftRadius", "BottomRightRadius"]
        );
    }

    #[test]
    fn int64_to_content_none() {
        use rbx_types::ContentType;

        let migration = serde_json::from_str::<PropertyMigration>(
            r#"{"To":["ObviouslyFakeProperty"],"Migration":"Int64ToContent"}"#,
        )
        .unwrap();
        let new_value = migration.perform(&0i64.into()).unwrap();

        match new_value {
            Variant::Content(content) => match content.value() {
                ContentType::None => {}
                other => panic!("expected ContentType::None, got {:?}", other),
            },
            other => {
                panic!("expected Variant::Content, got Variant::{:?}", other.ty())
            }
        }

        let new_value_2 = migration.perform(&Variant::Int64(-1)).unwrap();

        match new_value_2 {
            Variant::Content(content) => match content.value() {
                ContentType::None => {}
                other => panic!("expected ContentType::None, got {:?}", other),
            },
            other => {
                panic!("expected Variant::Content, got Variant::{:?}", other.ty())
            }
        }
    }

    #[test]
    fn int64_to_content_some() {
        use rbx_types::ContentType;

        let migration = serde_json::from_str::<PropertyMigration>(
            r#"{"To":["ObviouslyFakeProperty"],"Migration":"Int64ToContent"}"#,
        )
        .unwrap();
        let new_value = migration.perform(&1337i64.into()).unwrap();

        match new_value {
            Variant::Content(content) => match content.value() {
                ContentType::Uri(uri) => assert_eq!(uri, "rbxassetid://1337"),
                other => panic!(
                    "expected ContentType::Uri(\"rbxassetid://1337\"), got {:?}",
                    other
                ),
            },
            other => {
                panic!("expected Variant::Content, got Variant::{:?}", other.ty())
            }
        }
    }
}
