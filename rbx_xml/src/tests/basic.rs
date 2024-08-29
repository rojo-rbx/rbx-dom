//! Basic functionality tests

use rbx_dom_weak::types::{
    Attributes, BinaryString, BrickColor, Color3, Color3uint8, ColorSequence,
    ColorSequenceKeypoint, Enum, EnumItem, Font, MaterialColors, NumberRange, NumberSequence,
    NumberSequenceKeypoint, Rect, Tags, TerrainMaterials, UDim, UDim2, UniqueId, Variant,
    VariantType, Vector2, Vector3,
};
use rbx_dom_weak::{ustr, InstanceBuilder, WeakDom};

#[test]
fn with_bool() {
    let _ = env_logger::try_init();

    let document = r#"
        <roblox version="4">
            <Item class="BoolValue" referent="hello">
                <Properties>
                    <bool name="Value">true</bool>
                </Properties>
            </Item>
        </roblox>
    "#;

    let tree = crate::from_str_default(document).unwrap();

    let root = tree.root();
    let child = tree.get_by_ref(root.children()[0]).unwrap();

    assert_eq!(child.name, "BoolValue");
    assert_eq!(child.class, "BoolValue");
    assert_eq!(
        child.properties.get(&"Value".into()),
        Some(&Variant::Bool(true))
    );
}

#[test]
fn read_tags() {
    let _ = env_logger::try_init();

    let document = r#"
        <roblox version="4">
            <Item class="Folder" referent="hello">
                <Properties>
                    <BinaryString name="Tags">SGVsbG8AV29ybGQ=</BinaryString>
                </Properties>
            </Item>
        </roblox>
    "#;

    let dom = crate::from_str_default(document).unwrap();
    let folder = dom.get_by_ref(dom.root().children()[0]).unwrap();

    let mut tags = Tags::new();
    tags.push("Hello");
    tags.push("World");

    assert_eq!(
        folder.properties.get(&"Tags".into()),
        Some(&Variant::Tags(tags))
    );
}

#[test]
fn write_empty_tags() {
    let _ = env_logger::try_init();

    let part = InstanceBuilder::new("Part").with_property("Tags", Tags::new());
    let dom = WeakDom::new(part);

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &dom, &[dom.root_ref()]).unwrap();
    insta::assert_snapshot!(std::str::from_utf8(&encoded).unwrap());
}

#[test]
fn write_tags() {
    let _ = env_logger::try_init();

    let mut tags = Tags::new();
    tags.push("Hello");
    tags.push("World");

    let part = InstanceBuilder::new("Part").with_property("Tags", tags);
    let dom = WeakDom::new(part);

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &dom, &[dom.root_ref()]).unwrap();
    insta::assert_snapshot!(std::str::from_utf8(&encoded).unwrap());
}

#[test]
fn read_attributes() {
    let _ = env_logger::try_init();

    let document = r#"
        <roblox version="4">
            <Item class="Folder" referent="RBX10E3276249364E44B1EBE3BF36E14C1D">
                <Properties>
                    <BinaryString name="AttributesSerialize"><![CDATA[DwAAAAMAAABOYU4GAAAAAAAA+P8IAAAASW5maW5pdHkGAAAAAAAA8H8NAAAAQ29sb3JTZXF1
        ZW5jZRkDAAAAAAAAAAAAAAAAAIA/AAAAAAAAAAAAAAAAAAAAPwAAAAAAAIA/AAAAAAAAAAAA
        AIA/AAAAAAAAAAAAAIA/BwAAAFZlY3RvcjMRAACAPwAAAEAAAEBABwAAAFZlY3RvcjIQAAAg
        QQAASEIOAAAATnVtYmVyU2VxdWVuY2UXAwAAAAAAAAAAAAAAAACAPwAAAAAAAAA/AAAAAAAA
        AAAAAIA/AACAPwYAAABDb2xvcjMPo6IiPwAAAAAAAIA/CgAAAEJyaWNrQ29sb3IO7AMAAAQA
        AABSZWN0HAAAgD8AAABAAABAQAAAgEAFAAAAVURpbTIKAAAAPwoAAAAzMzM/HgAAAAQAAABV
        RGltCQAAAD9kAAAACwAAAE51bWJlclJhbmdlGwAAoEAAACBBBgAAAE51bWJlcgYAAAAAgBzI
        QAcAAABCb29sZWFuAwEGAAAAU3RyaW5nAg0AAABIZWxsbywgd29ybGQh]]></BinaryString>
                    <string name="Name">Folder</string>
                </Properties>
            </Item>
        </roblox>
    "#;

    let dom = crate::from_str_default(document).unwrap();
    let folder = dom.get_by_ref(dom.root().children()[0]).unwrap();

    assert_eq!(folder.properties.get(&"AttributesSerialize".into()), None);
    let folder_attributes = match folder.properties.get(&"Attributes".into()) {
        Some(Variant::Attributes(attrs)) => attrs,
        Some(other) => panic!(
            "Attributes property was not Attributes, it was: {:?}",
            other.ty()
        ),
        None => panic!("Attributes property was missing"),
    };

    let mut attributes = Attributes::new();
    attributes.insert("Boolean".into(), true.into());
    attributes.insert("BrickColor".into(), BrickColor::ReallyRed.into());
    attributes.insert("Color3".into(), Color3::new(162.0 / 255.0, 0.0, 1.0).into());
    attributes.insert(
        "ColorSequence".into(),
        ColorSequence {
            keypoints: vec![
                ColorSequenceKeypoint {
                    time: 0.0,
                    color: Color3::new(1.0, 0.0, 0.0),
                },
                ColorSequenceKeypoint {
                    time: 0.5,
                    color: Color3::new(0.0, 1.0, 0.0),
                },
                ColorSequenceKeypoint {
                    time: 1.0,
                    color: Color3::new(0.0, 0.0, 1.0),
                },
            ],
        }
        .into(),
    );
    attributes.insert("Number".into(), 12345.0f64.into());
    attributes.insert("NumberRange".into(), NumberRange::new(5.0, 10.0).into());
    attributes.insert(
        "NumberSequence".into(),
        NumberSequence {
            keypoints: vec![
                NumberSequenceKeypoint {
                    time: 0.0,
                    value: 1.0,
                    envelope: 0.0,
                },
                NumberSequenceKeypoint {
                    time: 0.5,
                    value: 0.0,
                    envelope: 0.0,
                },
                NumberSequenceKeypoint {
                    time: 1.0,
                    value: 1.0,
                    envelope: 0.0,
                },
            ],
        }
        .into(),
    );
    attributes.insert(
        "Rect".into(),
        Rect::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)).into(),
    );
    attributes.insert(
        "String".into(),
        BinaryString::from(&b"Hello, world!"[..]).into(),
    );
    attributes.insert("UDim".into(), UDim::new(0.5, 100).into());
    attributes.insert(
        "UDim2".into(),
        UDim2::new(UDim::new(0.5, 10), UDim::new(0.7, 30)).into(),
    );
    attributes.insert("Vector2".into(), Vector2::new(10.0, 50.0).into());
    attributes.insert("Vector3".into(), Vector3::new(1.0, 2.0, 3.0).into());
    attributes.insert("Infinity".into(), f64::INFINITY.into());

    for (key, value) in attributes {
        assert_eq!(folder_attributes.get(key), Some(&value));
    }
}

#[test]
fn write_material_colors() {
    let _ = env_logger::try_init();

    let terrain =
        InstanceBuilder::new("Terrain").with_property("MaterialColors", MaterialColors::new());
    let dom = WeakDom::new(terrain);

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &dom, &[dom.root_ref()]).unwrap();
    insta::assert_snapshot!(std::str::from_utf8(&encoded).unwrap());
}

#[test]
fn read_material_colors() {
    let _ = env_logger::try_init();

    let document = r#"
        <roblox version="4">
            <Item class="Terrain" referent="hope your day is swell :-)">
                <Properties>
                    <BinaryString name="MaterialColors">AAAAAAAAAQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyAhIiMkJSYnKCkqKywtLi8wMTIzNDU2Nzg5Ojs8PT4/</BinaryString>
                </Properties>
            </Item>
        </roblox>
    "#;

    let dom = crate::from_str_default(document).unwrap();
    let terrain = dom.get_by_ref(dom.root().children()[0]).unwrap();

    if let Some(Variant::MaterialColors(colors)) = terrain.properties.get(&"MaterialColors".into())
    {
        // There are tests to ensure competency in the actual MaterialColors
        // implementation, so these are just basic "are you ok" checks.
        assert_eq!(
            colors.get_color(TerrainMaterials::Grass).unwrap(),
            Color3uint8::new(1, 2, 3)
        );
        assert_eq!(
            colors.get_color(TerrainMaterials::CrackedLava).unwrap(),
            Color3uint8::new(40, 41, 42)
        );
        assert_eq!(
            colors.get_color(TerrainMaterials::Limestone).unwrap(),
            Color3uint8::new(58, 59, 60)
        );
    } else {
        panic!(
            "MaterialColors was not Some(Variant::MaterialColors(_)) and was instead {:?}",
            terrain.properties.get(&"MaterialColors".into())
        )
    }
}

#[test]
fn read_unique_id() {
    let _ = env_logger::try_init();

    let document = r#"
        <roblox version="4">
            <Item class="Workspace" referent="RBX10E3276249364E44B1EBE3BF36E14C1D">
                <Properties>
                    <UniqueId name="UniqueId">44b188dace632b4702e9c68d004815fc</UniqueId>
                    <bool name="Archivable">true</bool>
                    <string name="Name">Workspace</string>
                </Properties>
            </Item>
        </roblox>
    "#;

    let tree = crate::from_str(
        document,
        crate::DecodeOptions::new()
            // This is necessary at the moment because we do not actually
            // have UniqueId properties in our reflection database. This may
            // change, but it should in general be safe.
            .property_behavior(crate::DecodePropertyBehavior::ReadUnknown),
    )
    .unwrap();

    let root = tree.root();
    let child = tree.get_by_ref(root.children()[0]).unwrap();

    assert_eq!(child.name, "Workspace");
    assert_eq!(child.class, "Workspace");

    assert_eq!(
        child.properties.get(&"UniqueId".into()),
        Some(&Variant::UniqueId(UniqueId::new(
            0x0048_15fc,
            0x02e9_c68d,
            0x44b1_88da_ce63_2b47,
        )))
    );
}

#[test]
fn number_widening() {
    let _ = env_logger::try_init();
    let document = r#"
        <roblox version="4">
            <Item class="IntValue" referent="Test">
                <Properties>
                    <int name="Value">194</int>
                </Properties>
            </Item>
            <Item class="NumberValue" referent="Test">
                <Properties>
                    <float name="Value">1337</float>
                </Properties>
            </Item>
        </roblox>
    "#;
    let tree = crate::from_str_default(document).unwrap();

    let int_value = tree.get_by_ref(tree.root().children()[0]).unwrap();
    assert_eq!(int_value.class, "IntValue");
    assert_eq!(
        int_value.properties.get(&"Value".into()),
        Some(&Variant::Int64(194))
    );
    let float_value = tree.get_by_ref(tree.root().children()[1]).unwrap();
    assert_eq!(float_value.class, "NumberValue");
    assert_eq!(
        float_value.properties.get(&"Value".into()),
        Some(&Variant::Float64(1337.0))
    );
}

#[test]
fn migrated_properties() {
    let tree = WeakDom::new(InstanceBuilder::new("Folder").with_children([
        InstanceBuilder::new("ScreenGui").with_property("ScreenInsets", Enum::from_u32(0)),
        InstanceBuilder::new("ScreenGui").with_property("IgnoreGuiInset", true),
        InstanceBuilder::new("Part").with_property("Color", Color3::new(1.0, 1.0, 1.0)),
        InstanceBuilder::new("Part").with_property("BrickColor", BrickColor::Alder),
        InstanceBuilder::new("Part").with_property("brickColor", BrickColor::Alder),
        InstanceBuilder::new("TextLabel").with_property("FontFace", Font::default()),
        InstanceBuilder::new("TextLabel").with_property("Font", Enum::from_u32(8)),
    ]));

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &tree, &[tree.root_ref()]).unwrap();
    insta::assert_snapshot!(std::str::from_utf8(&encoded).unwrap());
}

#[test]
fn bad_migrated_property() {
    let tree = WeakDom::new(InstanceBuilder::new("Folder").with_children([
        InstanceBuilder::new("TextLabel").with_property("Font", Enum::from_u32(u32::MAX)),
    ]));

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &tree, &[tree.root_ref()]).unwrap();
    insta::assert_snapshot!(std::str::from_utf8(&encoded).unwrap());
}

#[test]
fn enum_item_to_enum() {
    let tree = WeakDom::new(InstanceBuilder::new("Part").with_property(
        "Material",
        EnumItem {
            ty: "Material".into(),
            value: 256,
        },
    ));

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &tree, &[tree.root_ref()]).unwrap();

    let decoded = crate::from_reader_default(encoded.as_slice()).unwrap();
    let prop_type = decoded
        .get_by_ref(*decoded.root().children().first().unwrap())
        .unwrap()
        .properties
        .get(&ustr("Material"))
        .unwrap()
        .ty();

    assert_eq!(prop_type, VariantType::Enum);
}
