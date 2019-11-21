use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    io::{self, Write},
    u32,
};

use byteorder::{LittleEndian, WriteBytesExt};
use rbx_dom_weak::{RbxId, RbxTree, RbxValue, RbxValueType};

use crate::{
    chunk::{ChunkBuilder, Compression},
    core::{id_from_value_type, RbxWriteExt, FILE_MAGIC_HEADER, FILE_SIGNATURE, FILE_VERSION},
    types_new::{BinaryType, BoolType, StringType},
};

static FILE_FOOTER: &[u8] = b"</roblox>";

pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], output: W) -> io::Result<()> {
    let mut serializer = BinarySerializer::new(tree, output);

    for id in ids {
        serializer.add_instance(*id);
    }

    log::debug!("Type info discovered: {:#?}", serializer.type_infos);

    serializer.generate_referents();

    log::trace!("Referents constructed: {:#?}", serializer.id_to_referent);

    serializer.write_header()?;
    serializer.serialize_metadata()?;
    serializer.serialize_instances()?;
    serializer.serialize_properties()?;
    serializer.serialize_parents()?;
    serializer.serialize_end()?;

    Ok(())
}

/// Represents all of the state during a single serialization session. A new
/// `BinarySerializer` object should be created every time we want to serialize
/// a binary model file.
struct BinarySerializer<'a, W> {
    /// The tree containing all of the instances that we're serializing.
    tree: &'a RbxTree,

    /// Where the binary output should be written.
    output: W,

    /// All of the instances, in a deterministic order, that we're going to be
    /// serializing.
    relevant_instances: Vec<RbxId>,

    /// A map from rbx-dom's unique instance ID (RbxId) to the ID space used in
    /// the binary model format, signed integers.
    id_to_referent: HashMap<RbxId, i32>,

    /// All of the types of instance discovered by our serializer that we'll be
    /// writing into the output.
    ///
    /// These are stored sorted so that we naturally iterate over them in order
    /// and improve our chances of being deterministic.
    type_infos: BTreeMap<String, TypeInfo>,

    /// The next type ID that should be assigned if a type is discovered and
    /// added to the serializer.
    next_type_id: u32,
}

/// An instance class that our serializer knows about. We should have one struct
/// per unique ClassName.
#[derive(Debug)]
struct TypeInfo {
    /// The ID that this serializer will use to refer to this type of instance.
    type_id: u32,

    /// The IDs of all of the instances of this type.
    object_ids: Vec<RbxId>,

    /// All of the defined properties for this type found on any instance of
    /// this type.
    ///
    /// Stored in a sorted map to try to ensure that we write out properties in
    /// a deterministic order.
    properties: BTreeMap<String, PropInfo>,
}

#[derive(Debug)]
struct PropInfo {
    kind: RbxValueType,
    // TODO: Should we store the default value for this descriptor here?
}

impl<'a, W: Write> BinarySerializer<'a, W> {
    fn new(tree: &'a RbxTree, output: W) -> Self {
        BinarySerializer {
            tree,
            output,
            relevant_instances: Vec::new(),
            id_to_referent: HashMap::new(),
            type_infos: BTreeMap::new(),
            next_type_id: 0,
        }
    }

    /// Mark the given instance ID and all of its descendants as intended for
    /// serialization with this serializer.
    fn add_instance(&mut self, id: RbxId) {
        self.relevant_instances.push(id);
        self.collect_type_info(id);

        for descendant in self.tree.descendants(id) {
            self.relevant_instances.push(descendant.get_id());
            self.collect_type_info(descendant.get_id());
        }
    }

    /// Collect information about all the different types of instance and their
    /// properties.
    fn collect_type_info(&mut self, id: RbxId) {
        let instance = self
            .tree
            .get_instance(id)
            .expect("Instance did not exist in tree");

        let type_info = self.get_or_create_type_info(&instance.class_name);
        type_info.object_ids.push(id);

        for (prop_name, prop_value) in &instance.properties {
            let prop_type = prop_value.get_type();

            if let Some(prop_info) = type_info.properties.get(prop_name) {
                // TODO: We should be able to check if this value can be
                // converted into the correct type instead.
                assert_eq!(prop_type, prop_info.kind);
            } else {
                // TODO: Add configurability for using reflection information
                // and how rbx_binary should fall back when encountering unknown
                // properties.
                //
                // Currently we just use the type of the first copy of this
                // property that we find in the tree.

                type_info
                    .properties
                    .insert(prop_name.to_owned(), PropInfo { kind: prop_type });
            };
        }
    }

    fn get_or_create_type_info(&mut self, class_name: &str) -> &mut TypeInfo {
        if !self.type_infos.contains_key(class_name) {
            let type_id = self.next_type_id;
            self.next_type_id += 1;

            let type_info = TypeInfo::new(type_id);
            self.type_infos.insert(class_name.to_owned(), type_info);
        }

        self.type_infos.get_mut(class_name).unwrap()
    }

    /// Populate the map from rbx-dom's instance ID space to the IDs that we'll
    /// be serializing to the model.
    fn generate_referents(&mut self) {
        self.id_to_referent.reserve(self.relevant_instances.len());

        let mut next_referent = 0;

        for id in &self.relevant_instances {
            self.id_to_referent.insert(*id, next_referent);
            next_referent += 1;
        }
    }

    fn write_header(&mut self) -> io::Result<()> {
        self.write_all(FILE_MAGIC_HEADER)?;
        self.write_all(FILE_SIGNATURE)?;
        self.write_u16::<LittleEndian>(FILE_VERSION)?;

        self.write_u32::<LittleEndian>(self.type_infos.len() as u32)?;
        self.write_u32::<LittleEndian>(self.relevant_instances.len() as u32)?;
        self.write_u64::<LittleEndian>(0)?;

        Ok(())
    }

    /// Write out any metadata about this file, stored in a chunk named META.
    fn serialize_metadata(&mut self) -> io::Result<()> {
        // TODO: There is no concept of metadata in a tree yet.
        Ok(())
    }

    /// Write out the declarations of all instances, stored in a series of
    /// chunks named INST.
    fn serialize_instances(&mut self) -> io::Result<()> {
        for type_info in self.type_infos.values() {
            for (prop_name, prop_info) in &type_info.properties {
                let value_type_id = match id_from_value_type(prop_info.kind) {
                    Some(id) => id,
                    None => {
                        log::debug!(
                            "Prop type {:?} is not supported by rbx_binary",
                            prop_info.kind
                        );

                        // TODO: Make this an error, configurably?
                        continue;
                    }
                };

                let mut chunk = ChunkBuilder::new(b"PROP", Compression::Compressed);

                chunk.write_u32::<LittleEndian>(type_info.type_id)?;
                chunk.write_string(&prop_name)?;
                chunk.write_u8(value_type_id)?;

                let tree = &self.tree;
                let values = type_info.object_ids.iter().map(|id| {
                    let instance = tree.get_instance(*id).unwrap();

                    // We store the Name property in a different field for
                    // convenience, but when serializing to the binary model
                    // format we need to handle it just like other properties.
                    if prop_name == "Name" {
                        Cow::Owned(RbxValue::String {
                            value: instance.name.clone(),
                        })
                    } else {
                        // TODO: Fall back to default value for this property
                        // descriptor.
                        Cow::Borrowed(instance.properties.get(prop_name).unwrap())
                    }
                });

                match prop_info.kind {
                    RbxValueType::String => StringType::write_values(&mut self.output, values)?,
                    RbxValueType::Bool => BoolType::write_values(&mut self.output, values)?,
                    _ => unreachable!(),
                }

                chunk.dump(&mut self.output)?;
            }
        }

        Ok(())
    }

    /// Write out batch declarations of property values for the instances
    /// previously defined in the INST chunks. Property data is contained in
    /// chunks named PROP.
    fn serialize_properties(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Write out the hierarchical relations between instances, stored in a
    /// chunk named PRNT.
    fn serialize_parents(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Write the fixed, uncompressed end chunk used to verify that the file
    /// hasn't been truncated mistakenly. This chunk is named END\0, with a zero
    /// byte at the end.
    fn serialize_end(&mut self) -> io::Result<()> {
        let mut end = ChunkBuilder::new(b"END\0", Compression::Uncompressed);
        end.write_all(FILE_FOOTER)?;
        end.dump(self)?;

        Ok(())
    }
}

/// Convenience implementation that lets us write through to the underlying
/// output buffer by writing to the BinarySerializer to avoid referring to
/// `self.output` a bunch.
impl<W: Write> Write for BinarySerializer<'_, W> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.output.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl TypeInfo {
    fn new(type_id: u32) -> Self {
        TypeInfo {
            type_id,
            object_ids: Vec::new(),
            properties: BTreeMap::new(),
        }
    }
}
