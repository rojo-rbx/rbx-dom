use std::io::{Read, Write};

use rbx_dom_weak::{PhysicalProperties as RbxPhysicalProperties, RbxValue};

use crate::{
    core::XmlType,
    serializer::{EncodeError, XmlEventWriter},
    deserializer::{DecodeError, EventIterator},
};

pub struct PhysicalProperties;

impl XmlType<Option<RbxPhysicalProperties>> for PhysicalProperties {
    const XML_NAME: &'static str = "PhysicalProperties";

    fn write_xml<W: Write>(
        _writer: &mut XmlEventWriter<W>,
        _name: &str,
        _value: &Option<RbxPhysicalProperties>,
    ) -> Result<(), EncodeError> {
        // TODO: Serialize data once it exists

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        // TODO: Actually read properties

        reader.eat_unknown_tag()?;

        Ok(RbxValue::PhysicalProperties {
            value: None,
        })
    }
}