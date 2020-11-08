use std::io::{Read, Write};

use rbx_dom_weak::types::Faces;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for Faces {
    const XML_TAG_NAME: &'static str = "Faces";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_tag_characters("faces", self.bits())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let value = reader
            .read_tag_contents("faces")?
            .parse::<u8>()
            .map_err(|e| reader.error(e))?;

        Self::from_bits(value).ok_or_else(|| {
            reader.error(DecodeErrorKind::InvalidContent("Faces value out of range"))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util;

    #[test]
    fn serialize_faces() {
        test_util::test_xml_serialize(
            "<Faces name=\"foo\"><faces>63</faces></Faces>",
            &Faces::from_bits(63).unwrap(),
        )
    }

    #[test]
    fn deserialize_faces() {
        test_util::test_xml_deserialize(
            "<Faces name=\"foo\"><faces>4</faces></Faces>",
            &Faces::from_bits(4).unwrap(),
        )
    }

    #[test]
    fn round_trip_faces() {
        test_util::test_xml_round_trip(&Faces::all());
        test_util::test_xml_round_trip(&Faces::empty());
    }
}
