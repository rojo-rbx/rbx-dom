use std::io::{Read, Write};

use rbx_dom_weak::types::Enum;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for Enum {
    const XML_TAG_NAME: &'static str = "token";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_characters(self.to_u32())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let value: u32 = reader
            .read_characters()?
            .parse()
            .map_err(|e| reader.error(e))?;

        Ok(Enum::from_u32(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip() {
        test_util::test_xml_round_trip(&Enum::from_u32(4654321));
    }
}
