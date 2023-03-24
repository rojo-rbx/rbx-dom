use std::io::{Read, Write};

use rbx_dom_weak::types::UniqueId;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for UniqueId {
    const XML_TAG_NAME: &'static str = "UniqueId";

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let content = reader.read_characters()?;

        let random = i64::from_str_radix(&content[0..16], 16).map_err(|e| reader.error(e))?;
        let time = u32::from_str_radix(&content[16..24], 16).map_err(|e| reader.error(e))?;
        let index = u32::from_str_radix(&content[24..32], 16).map_err(|e| reader.error(e))?;

        Ok(UniqueId::new(index, time, random))
    }

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_value(&format!(
            "{:016x}{:08x}{:08x}",
            &self.random(),
            &self.time(),
            &self.index()
        ))
    }
}
