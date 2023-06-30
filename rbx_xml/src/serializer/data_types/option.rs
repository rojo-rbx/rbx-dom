use std::io;

use rbx_dom_weak::types::CFrame;

use super::{cframe_serializer, EncodeError, XmlWriter};

pub fn optional_cframe_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Option<CFrame>,
) -> Result<(), EncodeError> {
    if let Some(cframe) = value {
        // I'm unwilling to copy `cframe` for this.
        writer.start_element("CFrame").finalize()?;
        cframe_serializer(writer, cframe)?;
        writer.end_element("CFrame")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize_test;
    use rbx_dom_weak::types::{Matrix3, Vector3};

    #[test]
    fn optional_cframe_none() {
        serialize_test!(optional_cframe_serializer, None::<CFrame>, "");
    }

    #[test]
    fn optional_cframe_some() {
        serialize_test!(
            optional_cframe_serializer,
            Some(CFrame::new(
                Vector3::new(10.0, 20.0, 30.0),
                Matrix3 {
                    x: Vector3::new(f32::INFINITY, f32::NEG_INFINITY, 0.0),
                    y: Vector3::new(0.5, 1.5, 1.0),
                    z: Vector3::new(-0.5, -10.0, 0.15625)
                }
            )),
            r#"<CFrame>
  <X>10</X>
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
  <R22>0.15625</R22>
</CFrame>"#
        )
    }
}
