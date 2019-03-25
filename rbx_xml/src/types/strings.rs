use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct StringType;

impl XmlType<str> for StringType {
    const XML_NAME: &'static str = "string";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_NAME).attr("name", name))?;
        writer.write(XmlWriteEvent::characters(&value))?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_NAME)?;
        let value = read_event!(reader, XmlReadEvent::Characters(value) => RbxValue::String { value: value.to_owned() });
        reader.expect_end_with_name(Self::XML_NAME)?;

        Ok(value)
    }
}

pub struct ProtectedStringType;

impl XmlType<str> for ProtectedStringType {
    const XML_NAME: &'static str = "ProtectedString";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_NAME).attr("name", name))?;
        writer.write(XmlWriteEvent::characters(&value))?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_NAME)?;
        let value = read_event!(reader, XmlReadEvent::Characters(value) => RbxValue::String { value: value.to_owned() });
        reader.expect_end_with_name(Self::XML_NAME)?;

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_string() {
        let _ = env_logger::try_init();

        let test_value = "Hello,\n\tworld!\n";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        StringType::write_xml(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = StringType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::String {
            value: test_value.to_owned(),
        });
    }

    #[test]
    fn de_protected_string() {
        let _ = env_logger::try_init();

        let test_value = "Hello,\n\tworld!\n";
        let source = format!("<ProtectedString name=\"foo\">{}</ProtectedString>", test_value);

        let mut reader = EventIterator::from_source(source.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = ProtectedStringType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::String {
            value: test_value.to_owned(),
        });
    }
}