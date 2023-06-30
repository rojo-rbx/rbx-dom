use std::io::BufRead;

use rbx_dom_weak::types::CFrame;

use super::cframe_deserializer;
use crate::deserializer::{
    error::DecodeError,
    reader::{XmlData, XmlReader},
};

pub fn optional_cframe_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<Option<CFrame>, DecodeError> {
    match reader.peek() {
        Some(Ok(XmlData::ElementStart { name, .. })) if name == "CFrame" => {
            reader.expect_start_with_name("CFrame")?;
            let cf = cframe_deserializer(reader)?;
            reader.expect_end_with_name("CFrame")?;
            Ok(Some(cf))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize_test;
    use rbx_dom_weak::types::{Matrix3, Vector3};

    #[test]
    fn optional_cframe_none() {
        deserialize_test!(optional_cframe_deserializer, None::<CFrame>, "");
    }

    #[test]
    fn optional_cframe_some() {
        deserialize_test!(
            optional_cframe_deserializer,
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
