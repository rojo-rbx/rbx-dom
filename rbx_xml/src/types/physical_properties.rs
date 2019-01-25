use std::io::{Read, Write};

use rbx_tree::{PhysicalProperties, RbxValue};

use crate::{
    serializer::{EncodeError, XmlEventWriter},
    deserializer::{DecodeError, EventIterator},
};

pub fn deserialize_physical_properties<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    // TODO: Actually read properties

    reader.eat_unknown_tag()?;

    Ok(RbxValue::PhysicalProperties {
        value: None,
    })
}

pub fn serialize_physical_properties<W: Write>(
    _writer: &mut XmlEventWriter<W>,
    _name: &str,
    _value: Option<PhysicalProperties>,
) -> Result<(), EncodeError> {
    // TODO: Serialize data once it exists

    Ok(())
}