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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize_test;
    use rbx_dom_weak::types::{Matrix3, Vector3};

    #[test]
    fn cframe() {
        deserialize_test!(
            cframe_deserializer,
            CFrame::new(
                Vector3::new(10.0, 20.0, 30.0),
                Matrix3 {
                    x: Vector3::new(f32::INFINITY, f32::NEG_INFINITY, 0.0),
                    y: Vector3::new(0.5, 1.5, 1.0),
                    z: Vector3::new(-0.5, -10.0, 0.15625)
                }
            ),
            r#"<X>10</X>
<Y>20</Y>
<Z>30</Z>
<R00>INF</R00>
<R01>-INF</R01>
<R02>0</R02>
<R10>0.5</R10>
<R11>1.5</R11>
<R12>1</R12>
<R20>-0.5</R20>
<R21>-10</R21>
<R22>0.15625</R22>"#
        )
    }
}
