use std::io::{Read, Write};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

impl XmlType for bool {
    const XML_TAG_NAME: &'static str = "bool";

    fn write_xml<W: Write>(
        &self,
        writer: &mut XmlEventWriter<W>,
        name: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        let value_as_str = if *self { "true" } else { "false" };

        writer.write(XmlWriteEvent::characters(value_as_str))?;
        writer.end_element()?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let content = reader.read_characters()?;

        let value = match content.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                return Err(reader.error(DecodeErrorKind::InvalidContent("expected true or false")))
            }
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_true() {
        test_util::test_xml_round_trip::<BoolType, _>(&true, RbxValue::Bool { value: true });
    }

    #[test]
    fn round_trip_false() {
        test_util::test_xml_round_trip::<BoolType, _>(&false, RbxValue::Bool { value: false });
    }
}
