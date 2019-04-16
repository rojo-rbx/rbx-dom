use std::{
    io::{self, Read, Write},
    mem,
};

use byteorder::WriteBytesExt;

mod string;
mod bool;

pub use self::string::*;
pub use self::bool::*;

fn encode_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

fn decode_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}

fn encode_interleaved_transformed_i32_array<W: Write, I>(
    output: &mut W,
    values: I,
) -> io::Result<()>
where
    I: Iterator<Item = i32> + Clone,
{
    for shift in &[24, 16, 8, 0] {
        for value in values.clone() {
            let encoded = encode_i32(value) >> shift;
            output.write_u8(encoded as u8)?;
        }
    }
    Ok(())
}

fn decode_interleaved_transformed_i32_array<R: Read>(
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

        output[i] = decode_i32((v0 << 24) | (v1 << 16) | (v2 << 8) | v3);
    }

    Ok(())
}

pub fn encode_referent_array<W: Write, I>(output: &mut W, values: I) -> io::Result<()>
where
    I: Iterator<Item = i32> + Clone,
{
    let mut last_value = 0;
    let delta_encoded = values
        .map(move |value| {
            let encoded = value - last_value;
            last_value = value;
            encoded
        });

    encode_interleaved_transformed_i32_array(output, delta_encoded)
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