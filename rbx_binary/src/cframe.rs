use rbx_dom_weak::types::{Matrix3, Vector3};

pub(crate) fn to_basic_rotation_id(matrix3: Matrix3) -> Option<u8> {
    let x_id = matrix3.x_row().to_normal_id()?;
    let y_id = matrix3.y_row().to_normal_id()?;
    let z_id = matrix3.z_row().to_normal_id()?;
    let basic_rotation_id = (6 * x_id) + y_id + 1;

    // Because we don't enforce orthonormality, it's still possible at
    // this point for the z row to differ from the basic rotation's z
    // row. Roblox will never output a matrix like this, but we check
    // for it anyway to avoid altering its value.
    if from_basic_rotation_id(basic_rotation_id)?
        .z_row()
        .to_normal_id()?
        == z_id
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
    for id in 0x02..0x24 {
        if let Some(rotation) = from_basic_rotation_id(id) {
            assert!(id == to_basic_rotation_id(rotation).unwrap())
        }
    }
}
