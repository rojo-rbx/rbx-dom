use std::io::{Read, Write};

use rbx_dom_weak::{RbxValue, ColorSequence, ColorSequenceKeypoint};

use crate::{
    core::XmlType,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    deserializer_core::{XmlEventReader},
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

pub struct ColorSequenceType;

impl XmlType<ColorSequence> for ColorSequenceType {
    const XML_TAG_NAME: &'static str = "ColorSequence";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &ColorSequence,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        for keypoint in &value.keypoints {
            writer.write_characters(keypoint.time)?;
            writer.write(XmlWriteEvent::characters(" "))?;
            writer.write_characters(keypoint.color[0])?;
            writer.write(XmlWriteEvent::characters(" "))?;
            writer.write_characters(keypoint.color[1])?;
            writer.write(XmlWriteEvent::characters(" "))?;
            writer.write_characters(keypoint.color[2])?;
            writer.write(XmlWriteEvent::characters(" "))?;

            // Envelope is always 0 for ColorSequenceKeypoint. This value isn't
            // exposed to developers but serializes in the XML format.
            writer.write_characters(0)?;
            writer.write(XmlWriteEvent::characters(" "))?;
        }

        writer.end_element()?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let contents = reader.read_characters()?;
        let mut pieces = contents
            .split(" ")
            .filter(|slice| !slice.is_empty())
            .map(|piece| {
                piece.parse::<f32>()
                    .map_err(|e| reader.error(e))
            });
        let mut keypoints = Vec::new();

        let wrong_length = || reader.error(DecodeErrorKind::InvalidContent("incorrect number of values"));

        loop {
            let time = match pieces.next() {
                Some(value) => value?,
                None => break,
            };

            let r = pieces.next().ok_or_else(wrong_length)??;
            let g = pieces.next().ok_or_else(wrong_length)??;
            let b = pieces.next().ok_or_else(wrong_length)??;

            // This value is always zero, isn't developer-exposed, and doesn't
            // have a corresponding field in rbx_dom_weak's type.
            let _envelope = pieces.next().ok_or_else(wrong_length)??;

            keypoints.push(ColorSequenceKeypoint { time, color: [r, g, b] });
        }

        if keypoints.len() < 2 {
            return Err(reader.error(DecodeErrorKind::InvalidContent("expected two or more keypoints")));
        }

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::ColorSequence {
            value: ColorSequence {
                keypoints,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_color_sequence() {
        let test_input = ColorSequence {
            keypoints: vec![
                ColorSequenceKeypoint {
                    time: 0.0,
                    color: [0.0, 0.5, 1.0],
                },
                ColorSequenceKeypoint {
                    time: 1.0,
                    color: [1.0, 0.5, 0.0],
                },
            ],
        };

        test_util::test_xml_round_trip::<ColorSequenceType, _>(
            &test_input,
            RbxValue::ColorSequence {
                value: test_input.clone(),
            }
        );
    }

    #[test]
    fn deserialize_color_sequence() {
        test_util::test_xml_deserialize::<ColorSequenceType, _>(
            r#"
                <ColorSequence name="foo">0 0 0.5 1 0 1 1 0.5 0 0 </ColorSequence>
            "#,
            RbxValue::ColorSequence {
                value: ColorSequence {
                    keypoints: vec![
                        ColorSequenceKeypoint {
                            time: 0.0,
                            color: [0.0, 0.5, 1.0],
                        },
                        ColorSequenceKeypoint {
                            time: 1.0,
                            color: [1.0, 0.5, 0.0],
                        },
                    ],
                },
            }
        );
    }

    #[test]
    fn serialize_color_sequence() {
        test_util::test_xml_serialize::<ColorSequenceType, _>(
            r#"
                <ColorSequence name="foo">0 0 0.5 1 0 1 1 0.5 0 0 </ColorSequence>
            "#,
            &ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint {
                        time: 0.0,
                        color: [0.0, 0.5, 1.0],
                    },
                    ColorSequenceKeypoint {
                        time: 1.0,
                        color: [1.0, 0.5, 0.0],
                    },
                ],
            }
        );
    }
}