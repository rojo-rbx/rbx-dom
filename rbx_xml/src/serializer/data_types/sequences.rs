use std::io;

use rbx_dom_weak::types::{ColorSequence, NumberRange, NumberSequence};

use super::{f32_serializer, EncodeError, XmlWriter};

pub fn color_sequence_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &ColorSequence,
) -> Result<(), EncodeError> {
    for keypoint in &value.keypoints {
        f32_serializer(writer, &keypoint.time)?;
        writer.write_text(" ")?;
        f32_serializer(writer, &keypoint.color.r)?;
        writer.write_text(" ")?;
        f32_serializer(writer, &keypoint.color.g)?;
        writer.write_text(" ")?;
        f32_serializer(writer, &keypoint.color.b)?;
        writer.write_text(" ")?;
        // The 'envelope' of a ColorSequenceKeypoint is always 0
        writer.write_text("0 ")?;
    }
    Ok(())
}

pub fn number_sequence_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &NumberSequence,
) -> Result<(), EncodeError> {
    for keypoint in &value.keypoints {
        f32_serializer(writer, &keypoint.time)?;
        writer.write_text(" ")?;
        f32_serializer(writer, &keypoint.value)?;
        writer.write_text(" ")?;
        f32_serializer(writer, &keypoint.envelope)?;
        writer.write_text(" ")?;
    }
    Ok(())
}

pub fn number_range_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &NumberRange,
) -> Result<(), EncodeError> {
    f32_serializer(writer, &value.min)?;
    writer.write_text(" ")?;
    f32_serializer(writer, &value.max)?;
    writer.write_text(" ")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rbx_dom_weak::types::{Color3, ColorSequenceKeypoint, NumberSequenceKeypoint};

    use super::*;
    use crate::serialize_test;

    #[test]
    fn color_sequence() {
        serialize_test!(
            color_sequence_serializer,
            ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint {
                        time: 0.0,
                        color: Color3::new(0.0, 0.5, 1.0)
                    },
                    ColorSequenceKeypoint {
                        time: 0.15625,
                        color: Color3::new(f32::INFINITY, f32::NEG_INFINITY, f32::NAN)
                    },
                    ColorSequenceKeypoint {
                        time: 1.0,
                        color: Color3::new(1.0, 0.5, 0.0)
                    }
                ]
            },
            "0 0 0.5 1 0 0.15625 INF -INF NAN 0 1 1 0.5 0 0 "
        )
    }

    #[test]
    fn number_sequence() {
        serialize_test!(
            number_sequence_serializer,
            NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint {
                        time: 0.0,
                        value: 10.5,
                        envelope: 11.5,
                    },
                    NumberSequenceKeypoint {
                        time: 0.15625,
                        value: f32::INFINITY,
                        envelope: -10.0,
                    },
                    NumberSequenceKeypoint {
                        time: 1.0,
                        value: 12.0,
                        envelope: 13.0,
                    },
                ],
            },
            "0 10.5 11.5 0.15625 INF -10 1 12 13 "
        )
    }

    #[test]
    fn number_range() {
        serialize_test!(
            number_range_serializer,
            NumberRange::new(80.5, -30.0),
            "80.5 -30 "
        )
    }
}
