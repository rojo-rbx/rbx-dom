use std::io::{Read, Write};

use rbx_dom_weak::types::CFrame;

use crate::{
    core::XmlType,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

impl XmlType for Option<CFrame> {
    const XML_TAG_NAME: &'static str = "OptionalCoordinateFrame";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        if let Some(cframe) = self {
            writer.write(XmlWriteEvent::start_element("CFrame"))?;
            cframe.write_xml(writer)?;
            writer.end_element()?;
        }

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        match reader.expect_peek()? {
            // If the next tag is an opening CFrame element, this value is Some
            // and we can attempt to deserialize a regular CFrame.
            XmlReadEvent::StartElement { name, .. } if name.local_name == "CFrame" => {
                reader.expect_start_with_name("CFrame")?;
                let inner = CFrame::read_xml(reader)?;
                reader.expect_end_with_name("CFrame")?;

                Ok(Some(inner))
            }

            // Otherwise, we expect there to be nothing else contained in here.
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rbx_dom_weak::types::{Matrix3, Vector3};

    use crate::test_util;

    #[test]
    fn round_trip_some() {
        let test_input = Some(CFrame::new(
            Vector3::new(123.0, 456.0, 789.0),
            Matrix3 {
                x: Vector3::new(987.0, 654.0, 432.0),
                y: Vector3::new(210.0, 0.0, -12345.0),
                z: Vector3::new(765.0, 234.0, 123123.0),
            },
        ));

        test_util::test_xml_round_trip(&test_input);
    }

    #[test]
    fn round_trip_none() {
        let test_input: Option<CFrame> = None;

        test_util::test_xml_round_trip(&test_input);
    }
}
