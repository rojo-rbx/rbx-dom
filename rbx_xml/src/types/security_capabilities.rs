use std::io::{Read, Write};

use rbx_dom_weak::types::SecurityCapabilities;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for SecurityCapabilities {
    const XML_TAG_NAME: &'static str = "SecurityCapabilities";

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        Ok(SecurityCapabilities::from_bits(
            reader
                .read_characters()?
                .parse()
                .map_err(|e| reader.error(e))?,
        ))
    }

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_characters(self.bits())
    }
}
