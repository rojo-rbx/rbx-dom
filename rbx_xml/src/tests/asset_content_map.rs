//! End-to-end tests for `AssetContentMap` properties using the default
//! encode/decode options, which only keep properties the reflection database
//! knows about.

use rbx_dom_weak::{
    types::{AssetContentMap, BinaryString, Variant},
    InstanceBuilder, WeakDom,
};

fn voxel_map() -> AssetContentMap {
    let mut map = AssetContentMap::new();
    map.insert(
        "L3;0,0,0;N4".to_owned(),
        BinaryString::from(b"VXFG\x02\x00\xff".to_vec()),
    );
    map.insert("L7;0,0,0;N4".to_owned(), BinaryString::new());
    map
}

/// Expanded Terrain stores its voxels in `Terrain.VoxelGridAssetContentMap`;
/// dropping it during a round trip deletes the terrain.
#[test]
fn terrain_voxel_grid_round_trip() {
    let tree = WeakDom::new(InstanceBuilder::new("Workspace").with_child(
        InstanceBuilder::new("Terrain").with_property("VoxelGridAssetContentMap", voxel_map()),
    ));

    let mut encoded = Vec::new();
    crate::to_writer_default(&mut encoded, &tree, &[tree.root_ref()]).unwrap();

    let text = std::str::from_utf8(&encoded).unwrap();
    assert!(
        text.contains(r#"<AssetContentMap name="VoxelGridAssetContentMap">"#),
        "VoxelGridAssetContentMap was not written:\n{}",
        text
    );

    let decoded = crate::from_reader_default(encoded.as_slice()).unwrap();
    let workspace = decoded.get_by_ref(decoded.root().children()[0]).unwrap();
    let terrain = decoded.get_by_ref(workspace.children()[0]).unwrap();

    assert_eq!(
        terrain.properties.get(&"VoxelGridAssetContentMap".into()),
        Some(&Variant::AssetContentMap(voxel_map())),
    );
}

/// The empty form Studio writes for terrain that is not using Expanded Terrain.
#[test]
fn terrain_empty_map_from_studio() {
    let source = r#"<roblox version="4">
        <Item class="Terrain" referent="RBX0">
            <Properties>
                <AssetContentMap name="VoxelGridAssetContentMap">[]</AssetContentMap>
            </Properties>
        </Item>
    </roblox>"#;

    let decoded = crate::from_str_default(source).unwrap();
    let terrain = decoded.get_by_ref(decoded.root().children()[0]).unwrap();

    assert_eq!(
        terrain.properties.get(&"VoxelGridAssetContentMap".into()),
        Some(&Variant::AssetContentMap(AssetContentMap::new())),
    );
}
