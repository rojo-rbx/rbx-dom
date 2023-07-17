use std::{collections::BTreeMap, fs, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use rbx_types::{
    Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
    ColorSequenceKeypoint, Content, CustomPhysicalProperties, Enum, Faces, Font, MaterialColors,
    Matrix3, NumberRange, NumberSequence, NumberSequenceKeypoint, PhysicalProperties, Ray, Rect,
    Region3int16, Tags, TerrainMaterials, UDim, UDim2, Variant, VariantType, Vector2, Vector2int16,
    Vector3, Vector3int16,
};
use serde::Serialize;

/// Generate a file with every kind of type into a file for debugging rbx_dom_lua.
#[derive(Debug, Parser)]
pub struct ValuesSubcommand {
    /// Where to output the file. The extension must be JSON (.json)
    pub output: PathBuf,
}

impl ValuesSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        if self.output.extension().unwrap_or_default() != "json" {
            bail!("The output path must have a .json extension")
        }

        let mut values: BTreeMap<&str, Variant> = BTreeMap::new();

        values.insert(
            "Attributes",
            Attributes::new()
                .with("TestBool", true)
                .with("TestString", "Test")
                .with("TestNumber", Variant::Float64(1337.0))
                .with("TestBrickColor", BrickColor::BrightYellow)
                .with("TestColor3", Color3::new(1.0, 0.5, 0.0))
                .with("TestVector2", Vector2::new(1.0, 2.0))
                .with("TestVector3", Vector3::new(1.0, 2.0, 3.0))
                .with(
                    "TestRect",
                    Rect::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)),
                )
                .with("TestUDim", UDim::new(1.0, 2))
                .with(
                    "TestUDim2",
                    UDim2::new(UDim::new(1.0, 2), UDim::new(3.0, 4)),
                )
                .into(),
        );
        values.insert("Axes", Axes::all().into());
        values.insert(
            "BinaryString",
            BinaryString::from("Hello!".as_bytes()).into(),
        );
        values.insert("Bool", true.into());
        values.insert("BrickColor", BrickColor::ReallyRed.into());
        values.insert(
            "CFrame",
            CFrame::new(
                Vector3::new(1.0, 2.0, 3.0),
                Matrix3::new(
                    Vector3::new(4.0, 5.0, 6.0),
                    Vector3::new(7.0, 8.0, 9.0),
                    Vector3::new(10.0, 11.0, 12.0),
                ),
            )
            .into(),
        );
        values.insert("Color3", Color3::new(1.0, 2.0, 3.0).into());
        values.insert("Color3uint8", Color3uint8::new(0, 128, 255).into());
        values.insert(
            "ColorSequence",
            ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0.0, Color3::new(1.0, 1.0, 0.5)),
                    ColorSequenceKeypoint::new(1.0, Color3::new(0.0, 0.0, 0.0)),
                ],
            }
            .into(),
        );
        values.insert("Content", Content::from("rbxassetid://12345").into());
        values.insert("Enum", Enum::from_u32(1234).into());
        values.insert("Faces", Faces::all().into());
        values.insert("Float32", 15.0f32.into());
        values.insert("Float64", 15123.0f64.into());
        values.insert("Font", Font::default().into());
        values.insert("Int32", 6014i32.into());
        values.insert("Int64", 23491023i64.into());
        values.insert(
            "MaterialColors",
            MaterialColors::from([
                (TerrainMaterials::Grass, Color3uint8::new(106, 127, 63)),
                (TerrainMaterials::Slate, Color3uint8::new(63, 127, 107)),
                (TerrainMaterials::Concrete, Color3uint8::new(127, 102, 63)),
                (TerrainMaterials::Brick, Color3uint8::new(138, 86, 62)),
                (TerrainMaterials::Sand, Color3uint8::new(143, 126, 95)),
                (TerrainMaterials::WoodPlanks, Color3uint8::new(139, 109, 79)),
                (TerrainMaterials::Rock, Color3uint8::new(102, 108, 111)),
                (TerrainMaterials::Glacier, Color3uint8::new(101, 176, 234)),
                (TerrainMaterials::Snow, Color3uint8::new(195, 199, 218)),
                (TerrainMaterials::Sandstone, Color3uint8::new(137, 90, 71)),
                (TerrainMaterials::Mud, Color3uint8::new(58, 46, 36)),
                (TerrainMaterials::Basalt, Color3uint8::new(30, 30, 37)),
                (TerrainMaterials::Ground, Color3uint8::new(102, 92, 59)),
                (
                    TerrainMaterials::CrackedLava,
                    Color3uint8::new(232, 156, 74),
                ),
                (TerrainMaterials::Asphalt, Color3uint8::new(115, 123, 107)),
                (
                    TerrainMaterials::Cobblestone,
                    Color3uint8::new(132, 123, 90),
                ),
                (TerrainMaterials::Ice, Color3uint8::new(129, 194, 224)),
                (TerrainMaterials::LeafyGrass, Color3uint8::new(115, 132, 74)),
                (TerrainMaterials::Salt, Color3uint8::new(198, 189, 181)),
                (TerrainMaterials::Limestone, Color3uint8::new(206, 173, 148)),
                (TerrainMaterials::Pavement, Color3uint8::new(148, 148, 140)),
            ])
            .into(),
        );
        values.insert("NumberRange", NumberRange::new(-36.0, 94.0).into());
        values.insert(
            "NumberSequence",
            NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0.0, 5.0, 2.0),
                    NumberSequenceKeypoint::new(1.0, 22.0, 0.0),
                ],
            }
            .into(),
        );
        values.insert(
            "Tags",
            Tags::from(vec![
                "foo".to_owned(),
                "con'fusion?!".to_owned(),
                "bar".to_owned(),
            ])
            .into(),
        );
        values.insert(
            "PhysicalProperties-Custom",
            PhysicalProperties::Custom(CustomPhysicalProperties {
                density: 0.5,
                friction: 1.0,
                elasticity: 0.0,
                friction_weight: 50.0,
                elasticity_weight: 25.0,
            })
            .into(),
        );
        values.insert(
            "PhysicalProperties-Default",
            PhysicalProperties::Default.into(),
        );
        values.insert(
            "Ray",
            Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0)).into(),
        );
        values.insert(
            "Rect",
            Rect::new(Vector2::new(0.0, 5.0), Vector2::new(10.0, 15.0)).into(),
        );
        values.insert(
            "Region3int16",
            Region3int16::new(Vector3int16::new(-10, -5, 0), Vector3int16::new(5, 10, 15)).into(),
        );
        values.insert("String", String::from("Hello, world!").into());
        values.insert("UDim", UDim::new(1.0, 32).into());
        values.insert(
            "UDim2",
            UDim2::new(UDim::new(-1.0, 100), UDim::new(1.0, -100)).into(),
        );
        values.insert("Vector2", Vector2::new(-50.0, 50.0).into());
        values.insert("Vector2int16", Vector2int16::new(-300, 300).into());
        values.insert("Vector3", Vector3::new(-300.0, 0.0, 1500.0).into());
        values.insert("Vector3int16", Vector3int16::new(60, 37, -450).into());

        let entries: BTreeMap<&str, TestEntry> = values
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    TestEntry {
                        ty: value.ty(),
                        value,
                    },
                )
            })
            .collect();

        fs::write(&self.output, serde_json::to_string_pretty(&entries)?)?;

        Ok(())
    }
}

#[derive(Serialize)]
struct TestEntry {
    value: Variant,
    ty: VariantType,
}
