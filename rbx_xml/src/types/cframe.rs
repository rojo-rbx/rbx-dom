use std::io::Read;

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
};

static TAG_NAMES: [&str; 12] = [ "X", "Y", "Z", "R00", "R01", "R02", "R10", "R11", "R12", "R20", "R21", "R22" ];

pub fn deserialize_cframe<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let mut components = [0.0; 12];

    for index in 0..12 {
        let tag_name = TAG_NAMES[index];
        components[index] = reader.read_tag_contents(tag_name)?.parse()?;
    }

    Ok(RbxValue::CoordinateFrame {
        value: components,
    })
}