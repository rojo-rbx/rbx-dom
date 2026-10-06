use std::io::{Read, Write};

use rbx_dom_weak::types::CFrame;

use crate::{
    core::XmlType,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::XmlEventWriter,
};

static TAG_NAMES: [&str; 12] = [
    "X", "Y", "Z", "R00", "R01", "R02", "R10", "R11", "R12", "R20", "R21", "R22",
];

impl XmlType for CFrame {
    const XML_TAG_NAME: &'static str = "CoordinateFrame";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        // FIXME: Should this be built into rbx_types?
        let as_slice = &[
            self.position.x,
            self.position.y,
            self.position.z,
            self.orientation.x.x,
            self.orientation.x.y,
            self.orientation.x.z,
            self.orientation.y.x,
            self.orientation.y.y,
            self.orientation.y.z,
            self.orientation.z.x,
            self.orientation.z.y,
            self.orientation.z.z,
        ];

        writer.write_tag_array(as_slice, &TAG_NAMES)?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let mut value = CFrame::identity();
        let mut seen = [false; TAG_NAMES.len()];

        // Older place files have been observed to serialize CFrame components
        // out of order, so match each child by its tag name rather than
        // assuming the canonical `TAG_NAMES` sequence. This lets those files
        // round-trip instead of failing with an "unexpected XML event" error.
        for _ in 0..TAG_NAMES.len() {
            // Look at the next child's name without consuming it, so
            // `read_value_in_tag` below can still read the whole element.
            let tag_name = match reader.expect_peek()? {
                XmlReadEvent::StartElement { name, .. } => name.local_name.clone(),
                _ => {
                    return Err(reader.error(DecodeErrorKind::InvalidContent(
                        "expected a CoordinateFrame component element",
                    )))
                }
            };

            let index = TAG_NAMES
                .iter()
                .position(|&candidate| candidate == tag_name)
                .ok_or_else(|| {
                    reader.error(DecodeErrorKind::InvalidContent(
                        "unexpected element in CoordinateFrame",
                    ))
                })?;

            if seen[index] {
                return Err(reader.error(DecodeErrorKind::InvalidContent(
                    "duplicate component in CoordinateFrame",
                )));
            }
            seen[index] = true;

            let component: f32 = reader.read_value_in_tag(&tag_name)?;
            match TAG_NAMES[index] {
                "X" => value.position.x = component,
                "Y" => value.position.y = component,
                "Z" => value.position.z = component,

                "R00" => value.orientation.x.x = component,
                "R01" => value.orientation.x.y = component,
                "R02" => value.orientation.x.z = component,

                "R10" => value.orientation.y.x = component,
                "R11" => value.orientation.y.y = component,
                "R12" => value.orientation.y.z = component,

                "R20" => value.orientation.z.x = component,
                "R21" => value.orientation.z.y = component,
                "R22" => value.orientation.z.z = component,

                _ => unreachable!(),
            }
        }

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;
    use rbx_dom_weak::types::{Matrix3, Vector3};

    #[test]
    fn round_trip() {
        let test_input = CFrame::new(
            Vector3::new(123.0, 456.0, 789.0),
            Matrix3 {
                x: Vector3::new(987.0, 654.0, 432.0),
                y: Vector3::new(210.0, 0.0, -12345.0),
                z: Vector3::new(765.0, 234.0, 123123.0),
            },
        );

        test_util::test_xml_round_trip(&test_input);
    }

    #[test]
    fn reads_components_in_any_order() {
        // The same CFrame as `round_trip`, but with the child components written
        // in a non-canonical order, as older place files have been observed to
        // do.
        let expected = CFrame::new(
            Vector3::new(123.0, 456.0, 789.0),
            Matrix3 {
                x: Vector3::new(987.0, 654.0, 432.0),
                y: Vector3::new(210.0, 0.0, -12345.0),
                z: Vector3::new(765.0, 234.0, 123123.0),
            },
        );

        let source = r#"
            <CoordinateFrame name="CFrame">
                <R11>0</R11>
                <Y>456</Y>
                <X>123</X>
                <R00>987</R00>
                <Z>789</Z>
                <R01>654</R01>
                <R02>432</R02>
                <R10>210</R10>
                <R12>-12345</R12>
                <R20>765</R20>
                <R21>234</R21>
                <R22>123123</R22>
            </CoordinateFrame>
        "#;

        test_util::test_xml_deserialize(source, &expected);
    }
}
