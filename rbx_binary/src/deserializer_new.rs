use std::{
    collections::HashMap,
    io::{self, Read},
};

use rbx_dom_weak::{RbxId, RbxInstanceProperties, RbxTree};

/// A compatibility shim to expose the new deserializer with the API of the old
/// deserializer.
pub fn decode_compat<R: Read>(
    tree: &mut RbxTree,
    parent_id: RbxId,
    mut source: R,
) -> io::Result<()> {
    let mut temp_tree = decode(source)?;
    let root_instance = temp_tree.get_instance(temp_tree.get_root_id()).unwrap();
    let root_children = root_instance.get_children_ids().to_vec();

    for id in root_children {
        temp_tree.move_instance(id, tree, parent_id);
    }

    Ok(())
}

pub fn decode<R: Read>(mut input: R) -> io::Result<RbxTree> {
    let deserializer = BinaryDeserializer::new(input);

    Ok(deserializer.finish())
}

struct BinaryDeserializer<R> {
    /// The input data encoded as a binary model.
    input: R,

    /// The tree that instances should be written into. Eventually returned to
    /// the user.
    tree: RbxTree,
}

impl<R: Read> BinaryDeserializer<R> {
    fn new(input: R) -> Self {
        let tree = make_temp_output_tree();

        BinaryDeserializer { input, tree }
    }

    fn finish(self) -> RbxTree {
        self.tree
    }
}

fn make_temp_output_tree() -> RbxTree {
    RbxTree::new(RbxInstanceProperties {
        name: "ROOT".to_owned(),
        class_name: "DataModel".to_owned(),
        properties: HashMap::new(),
    })
}
