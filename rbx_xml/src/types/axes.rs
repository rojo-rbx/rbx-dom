use std::io::{Read, Write};

use rbx_dom_weak::types::Axes;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for Axes {
    const XML_TAG_NAME: &'static str = "Axes";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_tag_characters("axes", self.bits())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let value = reader
            .read_tag_contents("axes")?
            .parse::<u8>()
            .map_err(|e| reader.error(e))?;

        Self::from_bits(value)
            .ok_or_else(|| reader.error(DecodeErrorKind::InvalidContent("Axes value out of range")))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util;

    #[test]
    fn serialize_axes() {
        test_util::test_xml_serialize(
            "<Axes name=\"foo\"><axes>5</axes></Axes>",
            &Axes::from_bits(5).unwrap(),
        )
    }

    #[test]
    fn deserialize_axes() {
        test_util::test_xml_deserialize(
            "<Axes name=\"foo\"><axes>3</axes></Axes>",
            &Axes::from_bits(3).unwrap(),
        )
    }

    #[test]
    fn round_trip_axes() {
        test_util::test_xml_round_trip(&Axes::all());
    }
}
