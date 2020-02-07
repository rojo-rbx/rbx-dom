//! Shim module to get rbx_xml compiling against rbx_dom_weak 2.0

use rbx_dom_weak::types::{Variant, VariantType};

pub enum TodoValueConversionType {
    Converted(Variant),
    Unnecessary,
    Failed,
}

pub trait TodoValueConversion {
    fn try_convert_ref(&self, target_type: VariantType) -> TodoValueConversionType {
        unimplemented!();
    }
}

impl TodoValueConversion for Variant {}
