use std::{
    io::{self, Read, Write},
    mem,
};

use byteorder::{ReadBytesExt, WriteBytesExt};
use rbx_dom_weak::RbxValue;

use crate::core::{untransform_i32, BinaryType, RbxReadExt, RbxWriteExt};

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

    fn write_array<'write, I, W: Write>(output: &mut W, values: I) -> io::Result<()>
    where
        I: Iterator<Item = &'write bool>,
    {
        for value in values {
            output.write_u8(*value as u8)?;
        }

        Ok(())
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

    fn write_array<'write, I, W: Write>(output: &mut W, values: I) -> io::Result<()>
    where
        I: Iterator<Item = &'write str>,
    {
        for value in values {
            output.write_string(value)?;
        }

        Ok(())
    }
}

pub fn decode_interleaved_transformed_i32_array<R: Read>(
    source: &mut R,
    output: &mut [i32],
) -> io::Result<()> {
    let mut buffer = vec![0; output.len() * mem::size_of::<i32>()];
    source.read_exact(&mut buffer)?;

    for i in 0..output.len() {
        let v0 = buffer[i] as i32;
        let v1 = buffer[i + output.len()] as i32;
        let v2 = buffer[i + output.len() * 2] as i32;
        let v3 = buffer[i + output.len() * 3] as i32;

        output[i] = untransform_i32((v0 << 24) | (v1 << 16) | (v2 << 8) | v3);
    }

    Ok(())
}

pub fn decode_referent_array<R: Read>(source: &mut R, output: &mut [i32]) -> io::Result<()> {
    decode_interleaved_transformed_i32_array(source, output)?;
    let mut last = 0;

    for i in 0..output.len() {
        output[i] += last;
        last = output[i];
    }

    Ok(())
}
