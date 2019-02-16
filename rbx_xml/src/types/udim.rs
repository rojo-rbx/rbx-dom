use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

static UDIM_TAGS: [&str; 2] = ["S", "O"];
static UDIM2_TAGS: [&str; 4] = ["XS", "XO", "YS", "YO"];

pub fn serialize_udim<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [f32; 2],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("UDim").attr("name", name))?;
	writer.write_tag_array(&value, &UDIM_TAGS)?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn serialize_udim2<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [f32; 4],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("UDim2").attr("name", name))?;
	writer.write_tag_array(&value, &UDIM2_TAGS)?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_udim<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("UDim")?;

	let s: f32 = reader.read_tag_contents("S")?.parse()?;
	let o: f32 = reader.read_tag_contents("O")?.parse()?;

	reader.expect_end_with_name("UDim")?;

	Ok(RbxValue::UDim {
		value: [s, o],
	})
}

pub fn deserialize_udim2<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("UDim2")?;

	let xs: f32 = reader.read_tag_contents("XS")?.parse()?;
	let xo: f32 = reader.read_tag_contents("XO")?.parse()?;
	let ys: f32 = reader.read_tag_contents("YS")?.parse()?;
	let yo: f32 = reader.read_tag_contents("YO")?.parse()?;

	reader.expect_end_with_name("UDim2")?;

	Ok(RbxValue::UDim2 {
		value: [xs, xo, ys, yo],
	})
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn round_trip_udim2() {
		let _ = env_logger::try_init();

		let test_input: [f32; 4] = [0.5, 0.0, 0.25, 0.0];
		let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_udim2(&mut writer, "foo", test_input).unwrap();

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_udim2(&mut reader).unwrap();

        assert_eq!(value, RbxValue::UDim2 {
            value: test_input,
        });
	}
}