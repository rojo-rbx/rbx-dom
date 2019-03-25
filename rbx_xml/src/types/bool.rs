use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
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
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let value = read_event!(reader, XmlReadEvent::Characters(content) => {
            match content.as_str() {
                "true" => true,
                "false" => false,
                _ => return Err(DecodeError::Message("invalid boolean value, expected true or false")),
            }
        });

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Bool {
            value
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip() {
        let _ = env_logger::try_init();

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        BoolType::write_xml(&mut writer, "foo", &true).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = BoolType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Bool {
            value: true,
        });
    }
}