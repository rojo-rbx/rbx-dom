use rbx_dom_weak::types::{Matrix3, Vector3};

pub(crate) fn to_basic_rotation_id(matrix3: Matrix3) -> Option<u8> {
    let right_id = matrix3.right_vector().to_normal_id()?;
    let up_id = matrix3.up_vector().to_normal_id()?;
    let back_id = matrix3.back_vector().to_normal_id()?;
    let basic_rotation_id = (6 * right_id) + up_id + 1;

    // Because we don't enforce orthonormality, it's still possible at
    // this point for the back vector to differ from the basic
    // rotation's back vector. Roblox will never output a matrix like
    // this, but we check for it anyway to avoid altering its value.
    if from_basic_rotation_id(basic_rotation_id)?
        .back_vector()
        .to_normal_id()?
        == back_id
    {
        Some(basic_rotation_id)
    } else {
        None
    }
}

pub(crate) fn from_basic_rotation_id(id: u8) -> Option<Matrix3> {
    let right = Vector3::from_normal_id((id - 1) / 6)?;
    let up = Vector3::from_normal_id((id - 1) % 6)?;
    let back = right.cross(up);

    Some(Matrix3::from_columns(right, up, back))
}

#[test]
fn basic_rotation_id_round_trip() {
    for id in 0x02..0x24 {
        if let Some(rotation) = from_basic_rotation_id(id) {
            if let Some(encoded) = to_basic_rotation_id(rotation) {
                assert!(encoded == id)
            }
        }
    }
}
