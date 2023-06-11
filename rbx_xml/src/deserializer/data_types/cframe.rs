//! Implements deserialization `CFrame`.

use std::io::BufRead;

use rbx_dom_weak::types::{CFrame, Matrix3, Vector3};

use super::f32_deserializer;
use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn cframe_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<CFrame, DecodeError> {
    let x = reader.read_named_with("X", f32_deserializer)?;
    let y = reader.read_named_with("Y", f32_deserializer)?;
    let z = reader.read_named_with("Z", f32_deserializer)?;
    let r00 = reader.read_named_with("R00", f32_deserializer)?;
    let r01 = reader.read_named_with("R01", f32_deserializer)?;
    let r02 = reader.read_named_with("R02", f32_deserializer)?;
    let r10 = reader.read_named_with("R10", f32_deserializer)?;
    let r11 = reader.read_named_with("R11", f32_deserializer)?;
    let r12 = reader.read_named_with("R12", f32_deserializer)?;
    let r20 = reader.read_named_with("R20", f32_deserializer)?;
    let r21 = reader.read_named_with("R21", f32_deserializer)?;
    let r22 = reader.read_named_with("R22", f32_deserializer)?;

    Ok(CFrame {
        position: Vector3::new(x, y, z),
        orientation: Matrix3 {
            x: Vector3::new(r00, r01, r02),
            y: Vector3::new(r10, r11, r12),
            z: Vector3::new(r20, r21, r22),
        },
    })
}
