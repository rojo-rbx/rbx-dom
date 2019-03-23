use std::io::{Read, Write};

use rbx_dom_weak::{PhysicalProperties, RbxValue};

use crate::{
    serializer::{EncodeError, XmlEventWriter},
    deserializer::{DecodeError, EventIterator},
};

pub fn deserialize<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    // TODO: Actually read properties

    reader.eat_unknown_tag()?;

    Ok(RbxValue::PhysicalProperties {
        value: None,
    })
}

pub fn serialize<W: Write>(
    _writer: &mut XmlEventWriter<W>,
    _name: &str,
    _value: Option<PhysicalProperties>,
) -> Result<(), EncodeError> {
    // TODO: Serialize data once it exists

    Ok(())
}