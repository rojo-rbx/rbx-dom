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
