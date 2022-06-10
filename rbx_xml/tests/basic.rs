//! Temporary tests while re-bootstrapping rbx_xml

use rbx_dom_weak::types::{
    Attributes, BinaryString, BrickColor, Color3, ColorSequence, ColorSequenceKeypoint,
    NumberRange, NumberSequence, NumberSequenceKeypoint, Rect, Tags, UDim, UDim2, Variant, Vector2,
    Vector3,
};
use rbx_dom_weak::{InstanceBuilder, WeakDom};

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

    let tree = rbx_xml::from_str_default(document).unwrap();

    let root = tree.root();
    let child = tree.get_by_ref(root.children()[0]).unwrap();

    assert_eq!(child.name, "BoolValue");
    assert_eq!(child.class, "BoolValue");
    assert_eq!(child.properties.get("Value"), Some(&Variant::Bool(true)));
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

    let dom = rbx_xml::from_str_default(document).unwrap();
    let folder = dom.get_by_ref(dom.root().children()[0]).unwrap();

    let mut tags = Tags::new();
    tags.push("Hello");
    tags.push("World");

    assert_eq!(folder.properties.get("Tags"), Some(&Variant::Tags(tags)));
}

#[test]
fn write_empty_tags() {
    let _ = env_logger::try_init();

    let part = InstanceBuilder::new("Part").with_property("Tags", Tags::new());
    let dom = WeakDom::new(part);

    let mut encoded = Vec::new();
    rbx_xml::to_writer_default(&mut encoded, &dom, &[dom.root_ref()]).unwrap();
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
    rbx_xml::to_writer_default(&mut encoded, &dom, &[dom.root_ref()]).unwrap();
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

    let dom = rbx_xml::from_str_default(document).unwrap();
    let folder = dom.get_by_ref(dom.root().children()[0]).unwrap();

    assert_eq!(folder.properties.get("AttributesSerialize"), None);
    let folder_attributes = match folder.properties.get("Attributes") {
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
