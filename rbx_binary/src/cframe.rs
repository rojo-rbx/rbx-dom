use rbx_dom_weak::types::{Matrix3, Vector3};

pub(crate) fn to_basic_rotation_id(matrix3: Matrix3) -> Option<u8> {
    let transpose = matrix3.transpose();
    let x_id = transpose.x.to_normal_id()?;
    let y_id = transpose.y.to_normal_id()?;
    let z_id = transpose.z.to_normal_id()?;
    let basic_rotation_id = (6 * x_id) + y_id + 1;

    // Because we don't enforce orthonormality, it's still possible at
    // this point for the z row to differ from the basic rotation's z
    // row. Roblox will never output a matrix like this, but we check
    // for it anyway to avoid altering its value.
    if from_basic_rotation_id(basic_rotation_id)?
        .transpose()
        .z
        .to_normal_id()?
        == z_id
    {
        Some(basic_rotation_id)
    } else {
        None
    }
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    return Vector3::new(
        a.y * b.z - b.y * a.z, 
        a.z * b.x - b.z * a.x,
        a.x * b.y - b.x * a.y,
    );
}
pub fn from_normal_id(id: u8) -> Option<Vector3> {
    if id > 5 {
        return None;
    }

    return Some(Vector3::new(
        match id { 0 => 1.0, 3 => -1.0, _ => 0.0 },
        match id { 1 => 1.0, 4 => -1.0, _ => 0.0 },
        match id { 2 => 1.0, 5 => -1.0, _ => 0.0 },
    ))
}

pub(crate) fn from_basic_rotation_id(raw_id: u8) -> Option<Matrix3> {
    let id = raw_id - 1;
    
    // Check if value is in-range and legal.
    if id >= 36 || id % 3 == (id / 6) % 3 {
        return None 
    }
    
    let x_col = from_normal_id(id / 6).unwrap();
    let y_col = from_normal_id(id % 6).unwrap();
    let z_col = cross(x_col, y_col);
    
    return Some(Matrix3::new(x_col, y_col, z_col));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_rotation_id_round_trip() {
        for id in 0x02..0x24 {
            if let Some(rotation) = from_basic_rotation_id(id) {
                assert!(id == to_basic_rotation_id(rotation).unwrap())
            }
        }
    }
}
