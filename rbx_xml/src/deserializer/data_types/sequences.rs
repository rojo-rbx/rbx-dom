//! Implements deserialization for types that are sequences of floats
//! like `NumberSequence`, `ColorSequence`, and `NumberRange`.

use std::io::BufRead;

use rbx_dom_weak::types::{
    Color3, ColorSequence, ColorSequenceKeypoint, NumberRange, NumberSequence,
    NumberSequenceKeypoint,
};

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn number_sequence_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<NumberSequence, DecodeError> {
    let content = reader.eat_text()?;
    let mut keypoints = Vec::new();

    let mut pieces = content.split(' ').filter(|s| !s.is_empty()).map(|piece| {
        piece.parse::<f32>().map_err(|err| {
            reader.error(format!(
                "could not get 32-bit float from `{content}` because {err}"
            ))
        })
    });

    let wrong_length =
        || reader.error("incorrect number of values (must be a multiple of 3 numbers)");

    // This is fairly shamelessly taken from the old implementation.
    loop {
        let time = match pieces.next() {
            Some(value) => value?,
            None => break,
        };

        let value = pieces.next().ok_or_else(wrong_length)??;
        let envelope = pieces.next().ok_or_else(wrong_length)??;

        keypoints.push(NumberSequenceKeypoint::new(time, value, envelope))
    }

    if keypoints.len() < 2 {
        Err(reader.error("expected two or more keypoints"))
    } else {
        Ok(NumberSequence { keypoints })
    }
}

pub fn color_sequence_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<ColorSequence, DecodeError> {
    let content = reader.eat_text()?;
    let mut keypoints = Vec::new();

    let mut pieces = content.split(' ').filter(|s| !s.is_empty()).map(|piece| {
        piece.parse::<f32>().map_err(|err| {
            reader.error(format!(
                "could not get 32-bit float from `{content}` because {err}"
            ))
        })
    });

    let wrong_length =
        || reader.error("incorrect number of values (must be a multiple of 5 numbers)");

    // This is fairly shamelessly taken from the old implementation.
    loop {
        let time = match pieces.next() {
            Some(value) => value?,
            None => break,
        };

        let r = pieces.next().ok_or_else(wrong_length)??;
        let g = pieces.next().ok_or_else(wrong_length)??;
        let b = pieces.next().ok_or_else(wrong_length)??;

        let _envelope = pieces.next().ok_or_else(wrong_length)??;

        keypoints.push(ColorSequenceKeypoint::new(time, Color3::new(r, g, b)))
    }

    if keypoints.len() < 2 {
        Err(reader.error("expected two or more keypoints"))
    } else {
        Ok(ColorSequence { keypoints })
    }
}

pub fn number_range_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<NumberRange, DecodeError> {
    let content = reader.eat_text()?;
    let mut pieces = content.split(' ').filter(|s| !s.is_empty()).map(|piece| {
        piece.parse::<f32>().map_err(|err| {
            reader.error(format!(
                "could not get 32-bit float from `{content}` because {err}"
            ))
        })
    });

    let min = pieces
        .next()
        .ok_or_else(|| reader.error("missing Min value"))??;
    let max = pieces
        .next()
        .ok_or_else(|| reader.error("missing Max value"))??;

    if pieces.next().is_some() {
        Err(reader.error("too many values"))
    } else {
        Ok(NumberRange::new(min, max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize_test;

    #[test]
    #[should_panic]
    fn color_sequence_one_keypoint() {
        deserialize_test!(
            color_sequence_deserializer,
            ColorSequence {
                keypoints: vec![ColorSequenceKeypoint::new(0.0, Color3::new(1.0, 2.0, 3.0))]
            },
            "0 0 1 2 3"
        )
    }

    #[test]
    #[should_panic]
    fn color_sequence_invalid_count() {
        deserialize_test!(
            color_sequence_deserializer,
            ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0.0, Color3::new(1.0, 2.0, 3.0)),
                    ColorSequenceKeypoint::new(1.0, Color3::new(3.0, 2.0, 1.0))
                ]
            },
            "0 1 2 3 1 3 2 1"
        )
    }

    #[test]
    #[should_panic]
    fn number_sequence_one_keypoint() {
        deserialize_test!(
            number_sequence_deserializer,
            NumberSequence {
                keypoints: vec![NumberSequenceKeypoint::new(0.0, 1.0, 2.0)]
            },
            "0 1 2"
        )
    }

    #[test]
    #[should_panic]
    fn number_sequence_invalid_count() {
        deserialize_test!(
            number_sequence_deserializer,
            NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0.0, 1.0, 2.0),
                    NumberSequenceKeypoint::new(1.0, 2.0, 3.0)
                ]
            },
            "0 1 2 1 2 3 4"
        )
    }

    #[test]
    #[should_panic]
    fn number_range_only_one() {
        deserialize_test!(
            number_range_deserializer,
            NumberRange::new(10.0, 20.0),
            "10"
        )
    }

    #[test]
    #[should_panic]
    fn number_range_too_many() {
        deserialize_test!(
            number_range_deserializer,
            NumberRange::new(10.0, 20.0),
            "10 20 30"
        )
    }
}
