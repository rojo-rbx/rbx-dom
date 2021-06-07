use rbx_dom_weak::{
    types::{Color3, Color3uint8, Ref, Region3, Vector3},
    InstanceBuilder, WeakDom,
};

use crate::{text_deserializer::DecodedModel, to_writer};

/// A basic test to make sure we can serialize the simplest instance: a Folder.
#[test]
fn just_folder() {
    let tree = WeakDom::new(InstanceBuilder::new("Folder"));
    let mut buffer = Vec::new();

    to_writer(&mut buffer, &tree, &[tree.root_ref()]).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that a tree containing some instances with a value and others
/// without will correctly fall back to (some) default value.
#[test]
fn partially_present() {
    let tree = WeakDom::new(InstanceBuilder::new("Folder").with_children(vec![
        // This instance's `Value` property should be preserved.
        InstanceBuilder::new("StringValue").with_property("Value", "Hello"),
        // This instance's `Value` property should be the empty string.
        InstanceBuilder::new("StringValue"),
    ]));

    let root_refs = tree.root().children();

    let mut buffer = Vec::new();
    to_writer(&mut buffer, &tree, root_refs).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that unknown properties get serialized on instances.
#[test]
fn unknown_property() {
    let tree =
        WeakDom::new(InstanceBuilder::new("Folder").with_property("WILL_NEVER_EXIST", "Hi, mom!"));

    let mut buffer = Vec::new();
    to_writer(&mut buffer, &tree, &[tree.root_ref()]).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that serializing a tree with an unimplemented property type returns
/// an error instead of panicking.
///
/// This test will need to be updated once we implement the type used here.
#[test]
fn unimplemented_type_known_property() {
    let tree = WeakDom::new(InstanceBuilder::new("UIListLayout").with_property(
        "Padding",
        Region3::new(Vector3::new(0.0, 0.0, 50.0), Vector3::new(0.0, 0.0, 50.0)),
    ));

    let mut buffer = Vec::new();
    let result = to_writer(&mut buffer, &tree, &[tree.root_ref()]);

    assert!(result.is_err());
}

/// Ensures that serializing a tree with an unimplemented property type AND an
/// unknown property descriptor returns an error instead of panicking.
///
/// Because rbx_binary has additional logic for falling back to values with no
/// known property descriptor, we should make sure that logic works.
///
/// This test will need to be updated once we implement the type used here.
#[test]
fn unimplemented_type_unknown_property() {
    let tree = WeakDom::new(InstanceBuilder::new("Folder").with_property(
        "WILL_NEVER_EXIST",
        Region3::new(Vector3::new(0.0, 0.0, 50.0), Vector3::new(0.0, 0.0, 50.0)),
    ));

    let mut buffer = Vec::new();
    let result = to_writer(&mut buffer, &tree, &[tree.root_ref()]);

    assert!(result.is_err());
}

/// Ensures that the serializer returns an error instead of panicking if we give
/// it an ID not present in the tree.
#[test]
fn unknown_id() {
    let tree = WeakDom::new(InstanceBuilder::new("Folder"));

    let mut buffer = Vec::new();
    let result = to_writer(&mut buffer, &tree, &[Ref::new()]);

    assert!(result.is_err());
}

/// Ensures that only one name for each logical property is serialized to a
/// file. Here, we use BasePart.Size and BasePart.size, which alias and both
/// serialize to BasePart.size.
///
/// For fun, we also have a part with no size property at all. It should default
/// to (4.0, 1.2, 2.0), a relic of Roblox's distant past.
#[test]
fn logical_properties_basepart_size() {
    let tree = WeakDom::new(
        InstanceBuilder::new("Folder")
            .with_child(
                InstanceBuilder::new("Part").with_property("Size", Vector3::new(1.0, 2.0, 3.0)),
            )
            .with_child(
                InstanceBuilder::new("Part").with_property("size", Vector3::new(4.0, 5.0, 6.0)),
            )
            .with_child(InstanceBuilder::new("Part")),
    );

    let mut buffer = Vec::new();
    to_writer(&mut buffer, &tree, tree.root().children()).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that all valid combinations of color property names and
/// value types are properly handled.
#[test]
fn part_color() {
    let tree = WeakDom::new(
        InstanceBuilder::new("Folder")
            .with_child(
                InstanceBuilder::new("Part")
                    .with_property("Color3uint8", Color3::new(-0.25, 0.5, 1.2)),
            )
            .with_child(
                InstanceBuilder::new("Part")
                    .with_property("Color3uint8", Color3uint8::new(25, 86, 254)),
            )
            .with_child(
                InstanceBuilder::new("Part").with_property("Color", Color3::new(0.0, 0.5, 1.0)),
            )
            .with_child(
                InstanceBuilder::new("Part").with_property("Color", Color3uint8::new(1, 30, 100)),
            ),
    );

    let mut buf = Vec::new();
    let _ = to_writer(&mut buf, &tree, tree.root().children());

    let decoded = DecodedModel::from_reader(buf.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}
