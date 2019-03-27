use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct EnumType;

impl XmlType<u32> for EnumType {
    const XML_TAG_NAME: &'static str = "token";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &u32,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_characters(*value)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        let value: u32 = reader.read_tag_contents(Self::XML_TAG_NAME)?.parse()?;

        Ok(RbxValue::Enum {
            value,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip() {
        test_util::test_xml_round_trip::<EnumType, _>(
            &4654321,
            RbxValue::Enum {
                value: 4654321,
            }
        );
    }
}