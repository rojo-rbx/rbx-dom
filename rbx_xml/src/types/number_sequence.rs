use std::io::{Read, Write};

use rbx_dom_weak::{RbxValue, NumberSequence, NumberSequenceKeypoint};

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
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
        let mut pieces = contents.split(" ").filter(|slice| !slice.is_empty());
        let mut keypoints = Vec::new();

        loop {
            let time: f32 = match pieces.next() {
                Some(value) => value.parse()?,
                None => break,
            };

            let value: f32 = pieces.next()
                .ok_or(DecodeError::Message("Malformed NumberSequence: wrong number of values"))?
                .parse()?;

            let envelope: f32 = pieces.next()
                .ok_or(DecodeError::Message("Malformed NumberSequence: wrong number of values"))?
                .parse()?;

            keypoints.push(NumberSequenceKeypoint { time, value, envelope });
        }

        if keypoints.len() < 2 {
            return Err(DecodeError::Message("Malformed NumberSequence: must have two or more keypoints"));
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