use rbx_dom_weak::types::{Matrix3, Vector3};

pub(crate) fn to_basic_rotation_id(matrix3: Matrix3) -> Option<u8> {
    let right_vector_id = matrix3.right_vector().into_normal_id()?;
    let up_vector_id = matrix3.up_vector().into_normal_id()?;
    let look_vector_id = matrix3.negative_look_vector().into_normal_id()?;

    let basic_rotation_id = (6 * right_vector_id) + up_vector_id + 1;
    let rotation = from_basic_rotation_id(basic_rotation_id)?;
    // This might be overkill?
    if rotation.right_vector().into_normal_id()? == right_vector_id
        && rotation.up_vector().into_normal_id()? == up_vector_id
        && rotation.negative_look_vector().into_normal_id()? == look_vector_id
    {
        Some(basic_rotation_id)
    } else {
        None
    }
}

pub(crate) fn from_basic_rotation_id(id: u8) -> Option<Matrix3> {
    match id {
        0x02 => Some(Matrix3::identity()),
        0x03 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x05 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x06 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x07 => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x09 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x0a => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x0c => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x0d => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x0e => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x10 => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x11 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x14 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x15 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x17 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x18 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x19 => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x1b => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x1c => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x1e => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x1f => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        0x20 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        0x22 => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        0x23 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        _ => None,
    }
}

#[test]
fn basic_rotation_id_round_trip() {
    for id in 0x02..0x23 {
        if let Some(rotation) = from_basic_rotation_id(id) {
            assert!(id == to_basic_rotation_id(rotation).unwrap())
        }
    }
}
