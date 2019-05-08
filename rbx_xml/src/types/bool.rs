use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::NewXmlType as XmlType,
    error::{EncodeError, DecodeError, DecodeErrorKind},
    deserializer_core::XmlEventReader,
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

pub struct BoolType;

impl XmlType<bool> for BoolType {
    const XML_TAG_NAME: &'static str = "bool";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &bool,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        let value_as_str = if *value {
            "true"
        } else {
            "false"
        };

        writer.write(XmlWriteEvent::characters(value_as_str))?;
        writer.end_element()?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let content = reader.read_characters()?;

        let value = match content.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(reader.error(DecodeErrorKind::InvalidContent("expected true or false")))
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Bool {
            value
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_true() {
        test_util::test_xml_round_trip::<BoolType, _>(
            &true,
            RbxValue::Bool {
                value: true,
            }
        );
    }

    #[test]
    fn round_trip_false() {
        test_util::test_xml_round_trip::<BoolType, _>(
            &false,
            RbxValue::Bool {
                value: false,
            }
        );
    }
}