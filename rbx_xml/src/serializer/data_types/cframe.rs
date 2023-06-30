use std::io;

use rbx_dom_weak::types::CFrame;

use super::{EncodeError, XmlWriter};

pub fn cframe_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &CFrame,
) -> Result<(), EncodeError> {
    writer.write_rbx("X", value.position.x)?;
    writer.write_rbx("Y", value.position.y)?;
    writer.write_rbx("Z", value.position.z)?;

    writer.write_rbx("R00", value.orientation.x.x)?;
    writer.write_rbx("R01", value.orientation.x.y)?;
    writer.write_rbx("R02", value.orientation.x.z)?;

    writer.write_rbx("R10", value.orientation.y.x)?;
    writer.write_rbx("R11", value.orientation.y.y)?;
    writer.write_rbx("R12", value.orientation.y.z)?;

    writer.write_rbx("R20", value.orientation.z.x)?;
    writer.write_rbx("R21", value.orientation.z.y)?;
    writer.write_rbx("R22", value.orientation.z.z)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize_test;
    use rbx_dom_weak::types::{Matrix3, Vector3};

    #[test]
    fn cframe() {
        serialize_test!(
            cframe_serializer,
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
