//! Converts between various variant types during serialization.
//! This is necessary for the sake of a few properties

use std::{borrow::Cow, convert::TryInto};

use rbx_dom_weak::types::{BrickColor, Variant, VariantType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("{0} is not a valid BrickColor")]
    IntToBrickColor(i32),
}

pub fn convert(from: Cow<Variant>, to: VariantType) -> Result<Cow<Variant>, ConversionError> {
    Ok(Cow::Owned(match (from.as_ref(), to) {
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
        _ => {
            log::debug!(
                "no conversion possible for '{:?} -> {to:?}', returning input",
                from.ty()
            );
            return Ok(from.clone());
        }
    }))
}
