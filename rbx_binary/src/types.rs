use std::io::{self, Read};

use byteorder::ReadBytesExt;
use rbx_dom_weak::RbxValue;

use crate::core::RbxReadExt;

pub trait BinaryType<T: ?Sized + 'static> {
    fn read_array<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<RbxValue>>;
}

pub struct BoolType;

impl BinaryType<bool> for BoolType {
    fn read_array<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<RbxValue>> {
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            let value = source.read_u8()? != 0;
            result.push(RbxValue::Bool { value });
        }

        Ok(result)
    }
}

pub struct StringType;

impl BinaryType<str> for StringType {
    fn read_array<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<RbxValue>> {
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            result.push(RbxValue::String {
                value: source.read_string()?,
            });
        }

        Ok(result)
    }
}
