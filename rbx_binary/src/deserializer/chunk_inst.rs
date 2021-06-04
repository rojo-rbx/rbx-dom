use rbx_dom_weak::InstanceBuilder;

use crate::core::RbxReadExt;

use super::{error::Error, Instance, TypeInfo};

pub(super) fn deserialize(mut chunk: &[u8]) -> Result<(), Error> {
    let type_id = chunk.read_le_u32()?;
    let type_name = chunk.read_string()?;
    let object_format = chunk.read_u8()?;
    let number_instances = chunk.read_le_u32()?;

    log::trace!(
        "INST chunk (type ID {}, type name {}, format {}, {} instances)",
        type_id,
        type_name,
        object_format,
        number_instances,
    );

    let mut referents = vec![0; number_instances as usize];
    chunk.read_referent_array(&mut referents)?;

    // TODO: Check object_format and check for service markers if it's 1?

    for &referent in &referents {
        self.instances_by_ref.insert(
            referent,
            Instance {
                builder: InstanceBuilder::new(&type_name),
                children: Vec::new(),
            },
        );
    }

    self.type_infos.insert(
        type_id,
        TypeInfo {
            type_id,
            type_name,
            referents,
        },
    );

    Ok(())
}
