use std::io::{Read, Write};

use rbx_dom_weak::{RbxValue, NumberSequence, NumberSequenceKeypoint};

use crate::{
    core::NewXmlType as XmlType,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    deserializer_core::XmlEventReader,
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

pub struct NumberSequenceType;

impl XmlType<NumberSequence> for NumberSequenceType {
    const XML_TAG_NAME: &'static str = "NumberSequence";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &NumberSequence,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        for keypoint in &value.keypoints {
            writer.write_characters(keypoint.time)?;
            writer.write(XmlWriteEvent::characters(" "))?;
            writer.write_characters(keypoint.value)?;
            writer.write(XmlWriteEvent::characters(" "))?;
            writer.write_characters(keypoint.envelope)?;
            writer.write(XmlWriteEvent::characters(" "))?;
        }

        writer.write(XmlWriteEvent::end_element())?;

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

            let value = pieces.next().ok_or_else(wrong_length)??;
            let envelope = pieces.next().ok_or_else(wrong_length)??;

            keypoints.push(NumberSequenceKeypoint { time, value, envelope });
        }

        if keypoints.len() < 2 {
            return Err(reader.error(DecodeErrorKind::InvalidContent("expected two or more keypoints")));
        }

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::NumberSequence {
            value: NumberSequence {
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
    fn round_trip_number_sequence() {
        let test_input = NumberSequence {
            keypoints: vec![
                NumberSequenceKeypoint {
                    time: 0.0,
                    value: 10.0,
                    envelope: 11.0,
                },
                NumberSequenceKeypoint {
                    time: 1.0,
                    value: 12.0,
                    envelope: 13.0,
                },
            ],
        };

        test_util::test_xml_round_trip::<NumberSequenceType, _>(
            &test_input,
            RbxValue::NumberSequence {
                value: test_input.clone(),
            }
        );
    }

    #[test]
    fn deserialize_number_sequence() {
        test_util::test_xml_deserialize::<NumberSequenceType, _>(
            r#"
                <NumberSequence name="foo">0 10.5 11.5 1 12 13 </NumberSequence>
            "#,
            RbxValue::NumberSequence {
                value: NumberSequence {
                    keypoints: vec![
                        NumberSequenceKeypoint {
                            time: 0.0,
                            value: 10.5,
                            envelope: 11.5,
                        },
                        NumberSequenceKeypoint {
                            time: 1.0,
                            value: 12.0,
                            envelope: 13.0,
                        },
                    ],
                },
            }
        );
    }

    #[test]
    fn serialize_number_sequence() {
        test_util::test_xml_serialize::<NumberSequenceType, _>(
            r#"
                <NumberSequence name="foo">0 10.5 11.5 1 12 13 </NumberSequence>
            "#,
            &NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint {
                        time: 0.0,
                        value: 10.5,
                        envelope: 11.5,
                    },
                    NumberSequenceKeypoint {
                        time: 1.0,
                        value: 12.0,
                        envelope: 13.0,
                    },
                ],
            }
        );
    }
}