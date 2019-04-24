use std::borrow::Cow;

use crate::value::{RbxValue, RbxValueType};

trait IntoRbxValue {
    fn into_rbx_value(self, value_type: RbxValueType) -> Option<RbxValue>;
}

macro_rules! impl_number_type {
    ($ty: ty) => {
        impl IntoRbxValue for $ty {
            fn into_rbx_value(self, value_type: RbxValueType) -> Option<RbxValue> {
                match value_type {
                    RbxValueType::Float32 => Some(RbxValue::Float32 { value: self as f32 }),
                    RbxValueType::Float64 => Some(RbxValue::Float64 { value: self as f64 }),
                    RbxValueType::Int32 => Some(RbxValue::Int32 { value: self as i32 }),
                    RbxValueType::Int64 => Some(RbxValue::Int64 { value: self as i64 }),
                    _ => None
                }
            }
        }
    };
}

impl_number_type!(f64);
impl_number_type!(f32);
impl_number_type!(i64);
impl_number_type!(i32);

impl IntoRbxValue for Vec<u8> {
    fn into_rbx_value(self, value_type: RbxValueType) -> Option<RbxValue> {
        match value_type {
            RbxValueType::BinaryString => Some(RbxValue::BinaryString { value: self }),
            _ => None
        }
    }
}

impl IntoRbxValue for String {
    fn into_rbx_value(self, value_type: RbxValueType) -> Option<RbxValue> {
        match value_type {
            RbxValueType::String => Some(RbxValue::String { value: self }),
            RbxValueType::Content => Some(RbxValue::Content { value: self }),
            _ => None
        }
    }
}

impl RbxValue {
    /// Attempts to convert the `RbxValue` into a new value with the given type.
    ///
    /// Is a no-op if the value is already of the correct type.
    ///
    /// If the conversion fails, the value will be given back in the `Err` case.
    pub fn try_convert(self, target_type: RbxValueType) -> Result<RbxValue, RbxValue> {
        if self.get_type() == target_type {
            return Ok(self)
        }

        match (self, target_type) {
            (RbxValue::Float32 { value }, RbxValueType::Float64) => Ok(RbxValue::Float64 { value: value as f64 }),
            (RbxValue::Float64 { value }, RbxValueType::Float32) => Ok(RbxValue::Float32 { value: value as f32 }),

            (RbxValue::Int32 { value }, RbxValueType::Int64) => Ok(RbxValue::Int64 { value: value as i64 }),
            (RbxValue::Int64 { value }, RbxValueType::Int32) => Ok(RbxValue::Int32 { value: value as i32 }),

            (this, _) => Err(this)
        }
    }

    /// Attempts to convert a reference to an `RbxValue` to a new value with the
    /// given type.
    ///
    /// Is a no-op (by returning `Some(Cow::Borrowed(_))`) if the value is
    /// already the right type.
    ///
    /// If the conversion wasn't successful, returns `None`.
    pub fn try_convert_ref<'a>(&'a self, target_type: RbxValueType) -> Option<Cow<'a, RbxValue>> {
        if self.get_type() == target_type {
            return Some(Cow::Borrowed(self))
        }

        // TODO: Reduce duplication with try_convert

        match (self, target_type) {
            (RbxValue::Float32 { value }, RbxValueType::Float64) => Some(Cow::Owned(RbxValue::Float64 { value: *value as f64 })),
            (RbxValue::Float64 { value }, RbxValueType::Float32) => Some(Cow::Owned(RbxValue::Float32 { value: *value as f32 })),

            (RbxValue::Int32 { value }, RbxValueType::Int64) => Some(Cow::Owned(RbxValue::Int64 { value: *value as i64 })),
            (RbxValue::Int64 { value }, RbxValueType::Int32) => Some(Cow::Owned(RbxValue::Int32 { value: *value as i32 })),

            (_this, _) => None
        }
    }
}