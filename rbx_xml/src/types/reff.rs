use std::io::{Read, Write};

use rbx_tree::{RbxId, RbxValue};

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

const REF_HEADER: &str = "RBX";

pub fn serialize_ref<W: Write>(
	writer: &mut XmlEventWriter<W>,
    name: &str,
    value: Option<RbxId>,
) -> Result<(), EncodeError> {
	writer.write(XmlWriteEvent::start_element("Ref").attr("name", name))?;
	writer.write(XmlWriteEvent::characters(&if let Some(id) = value {
		format!(
			"{}{}",
			REF_HEADER,
			id.to_string()
				.chars()
				.filter(|x| *x != '-')
				.map(|c| c.to_ascii_uppercase())
				.collect::<String>()
		)
	} else {
		"null".to_string()
	}))?;
	writer.write(XmlWriteEvent::end_element())?;

	Ok(())
}

pub fn deserialize_ref<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
	let ref_contents = reader.read_tag_contents("Ref")?;

	Ok(RbxValue::Ref {
		value: if ref_contents.starts_with(REF_HEADER) {
			RbxId::parse_str(&ref_contents[REF_HEADER.len()..])
		} else {
			None
		},
	})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_ref_some() {
        let _ = env_logger::try_init();

        let test_input: Option<RbxId> = RbxId::parse_str("1b9cdd1f-d088-4f76-bfe6-091c1731e1fb");
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_ref(&mut writer, "foo", test_input).unwrap();
        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_ref(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Ref {
            value: test_input,
        });
    }

	#[test]
    fn round_trip_ref_none() {
        let _ = env_logger::try_init();

        let test_input: Option<RbxId> = None;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_ref(&mut writer, "foo", test_input).unwrap();
        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_ref(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Ref {
            value: test_input,
        });
    }
}