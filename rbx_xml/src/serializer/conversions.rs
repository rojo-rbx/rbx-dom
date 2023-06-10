//! Converts between various variant types during serialization.
//! This is necessary for the sake of a few properties

use std::convert::TryInto;

use rbx_dom_weak::types::{Attributes, BrickColor, Error, Tags, Variant, VariantType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("no conversion possible")]
    NoConversion,
    #[error("{0} is not a valid BrickColor")]
    IntToBrickColor(i32),
    #[error("tags contained invalid UTF-8")]
    InvalidTags,
    #[error("could not read attributes because {0}")]
    InvalidAttribute(Error),
}

pub fn convert(from: &Variant, to: VariantType) -> Result<Variant, ConversionError> {
    Ok(match (&from, to) {
        (Variant::BrickColor(value), VariantType::Int32) => Variant::Int32(*value as i32),
        (Variant::Int32(value), VariantType::BrickColor) => {
            let new = (*value)
                .try_into()
                .map_err(|_| ConversionError::IntToBrickColor(*value))?;
            Variant::BrickColor(
                BrickColor::from_number(new).ok_or(ConversionError::IntToBrickColor(*value))?,
            )
        }
        (Variant::Color3(value), VariantType::Color3uint8) => Variant::Color3uint8((*value).into()),
        (Variant::BinaryString(value), VariantType::Tags) => {
            Variant::Tags(Tags::decode(value.as_ref()).map_err(|_| ConversionError::InvalidTags)?)
        }
        (Variant::BinaryString(value), VariantType::Attributes) => {
            let bytes: &[u8] = value.as_ref();

            Variant::Attributes(
                Attributes::from_reader(bytes).map_err(ConversionError::InvalidAttribute)?,
            )
        }
        _ => return Err(ConversionError::NoConversion),
    })
}
