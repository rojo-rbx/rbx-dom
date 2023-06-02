//! Converts between various variant types during deserialization.
//! This is necessary for the sake of a few properties

use std::convert::TryInto;

use rbx_dom_weak::types::{BrickColor, Variant, VariantType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("no conversion possible")]
    NoConversion,
    #[error("{0} is not a valid BrickColor")]
    IntToBrickColor(i32),
}

pub fn convert(mut from: &mut Variant, to: VariantType) -> Result<(), ConversionError> {
    *from = match (&mut from, to) {
        (Variant::Int32(value), VariantType::BrickColor) => {
            let new = (*value)
                .try_into()
                .map_err(|_| ConversionError::IntToBrickColor(*value))?;
            Variant::BrickColor(
                BrickColor::from_number(new).ok_or(ConversionError::IntToBrickColor(*value))?,
            )
        }
        _ => return Err(ConversionError::NoConversion),
    };

    Ok(())
}
