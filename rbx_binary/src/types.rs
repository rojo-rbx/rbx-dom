use std::{
    io::{self, Read, Write},
    mem,
};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

pub fn encode_bool<W: Write>(output: &mut W, value: bool) -> io::Result<()> {
    if value {
        output.write_u8(1)
    } else {
        output.write_u8(0)
    }
}

pub fn decode_bool<R: Read>(source: &mut R) -> io::Result<bool> {
    let input = source.read_u8()?;

    if input == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn encode_bool_array<W: Write>(output: &mut W, values: &[bool]) -> io::Result<()> {
    for &value in values {
        encode_bool(output, value)?;
    }

    Ok(())
}

pub fn decode_bool_array<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<bool>> {
    let mut result = Vec::new();

    for _ in 0..count {
        result.push(decode_bool(source)?);
    }

    Ok(result)
}

pub fn encode_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

pub fn decode_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}

pub fn encode_string<W: Write>(output: &mut W, value: &str) -> io::Result<()> {
    output.write_u32::<LittleEndian>(value.len() as u32)?;
    write!(output, "{}", value)
}

pub fn decode_string<R: Read>(source: &mut R) -> io::Result<String> {
    let length = source.read_u32::<LittleEndian>()?;

    let mut value = String::new();
    source.take(length as u64).read_to_string(&mut value)?;

    Ok(value)
}

pub fn encode_string_array<W: Write>(output: &mut W, values: &[&str]) -> io::Result<()> {
    for value in values {
        encode_string(output, value)?;
    }

    Ok(())
}

pub fn decode_string_array<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<String>> {
    let mut result = Vec::new();

    for _ in 0..count {
        result.push(decode_string(source)?);
    }

    Ok(result)
}

pub fn encode_interleaved_transformed_i32_array<W: Write, I>(output: &mut W, values: I) -> io::Result<()>
    where I: Iterator<Item = i32> + Clone
{
    for shift in &[24, 16, 8, 0] {
        for value in values.clone() {
            let encoded = encode_i32(value) >> shift;
            output.write_u8(encoded as u8)?;
        }
    }
    Ok(())
}

pub fn decode_interleaved_transformed_i32_array<R: Read>(source: &mut R, output: &mut [i32]) -> io::Result<()> {
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
    where I: Iterator<Item = i32> + Clone
{
    let mut delta_encoded = Vec::new();
    let mut last_value = 0;

    for value in values {
        delta_encoded.push(value - last_value);
        last_value = value;
    }

    encode_interleaved_transformed_i32_array(output, delta_encoded.iter().cloned())
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