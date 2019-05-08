use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::NewXmlType as XmlType,
    error::{EncodeError, DecodeError},
    deserializer_core::{XmlEventReader},
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

static TAG_NAMES: [&str; 12] = ["X", "Y", "Z", "R00", "R01", "R02", "R10", "R11", "R12", "R20", "R21", "R22"];

pub struct CFrameType;
type CFrameValue = [f32; 12];

impl XmlType<CFrameValue> for CFrameType {
    const XML_TAG_NAME: &'static str = "CoordinateFrame";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &CFrameValue,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_array(value, &TAG_NAMES)?;
        writer.end_element()?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let mut components = [0.0; 12];

        for index in 0..12 {
            let tag_name = TAG_NAMES[index];
            components[index] = reader.read_tag_contents(tag_name)?
                .parse().map_err(|e| reader.error(e))?;
        }

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::CFrame {
            value: components,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip() {
        let test_input: [f32; 12] = [
            123.0, 456.0, 789.0,
            987.0, 654.0, 432.0,
            210.0, 0.0, -12345.0,
            765.0, 234.0, 123123.0,
        ];

        test_util::test_xml_round_trip::<CFrameType, _>(
            &test_input,
            RbxValue::CFrame {
                value: test_input,
            }
        );
    }
}