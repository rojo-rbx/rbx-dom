use std::io::{Read, Write};

use rbx_dom_weak::types::EnumValue;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

impl XmlType for EnumValue {
    const XML_TAG_NAME: &'static str = "token";

    fn write_xml<W: Write>(
        &self,
        writer: &mut XmlEventWriter<W>,
        name: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_characters(self.to_u32())?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let value: u32 = reader
            .read_tag_contents(Self::XML_TAG_NAME)?
            .parse()
            .map_err(|e| reader.error(e))?;

        Ok(EnumValue::from_u32(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip() {
        test_util::test_xml_round_trip(&EnumValue::from_u32(4654321));
    }
}
