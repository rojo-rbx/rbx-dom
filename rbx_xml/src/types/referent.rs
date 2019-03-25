use std::io::{Read, Write};

use rbx_dom_weak::{RbxId, RbxValue};

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct RefType;

impl XmlType<Option<RbxId>> for RefType {
    const XML_NAME: &'static str = "Ref";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &Option<RbxId>,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_NAME).attr("name", name))?;

        match value {
            Some(value) => writer.write(XmlWriteEvent::characters(&value.to_string()))?,
            None => writer.write(XmlWriteEvent::characters("null"))?,
        }

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        let _ref_contents = reader.read_tag_contents(Self::XML_NAME)?;

        // TODO: Return a different type and use it to figure out the instance's
        // actual rbx_dom_weak ID, which is separate from Roblox refs.
        Ok(RbxValue::Ref { value: None })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_ref_none() {
        let _ = env_logger::try_init();

        let test_input: Option<RbxId> = None;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        RefType::write_xml(&mut writer, "foo", &test_input).unwrap();
        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = RefType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Ref {
            value: test_input,
        });
    }
}