use std::io::{self, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rbx_dom_weak::RbxValue;

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

pub trait BinaryType<T: ?Sized + 'static> {
    fn read_array<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<RbxValue>>;

    fn write_array<'write, I, W: Write>(output: &mut W, values: I) -> io::Result<()>
    where
        I: Iterator<Item = &'write T>;
}

pub fn read_string<R: Read>(input: &mut R) -> io::Result<String> {
    let length = input.read_u32::<LittleEndian>()?;

    let mut value = String::with_capacity(length as usize);
    input.take(length as u64).read_to_string(&mut value)?;

    Ok(value)
}

pub fn write_string<W: Write>(output: &mut W, value: &str) -> io::Result<()> {
    output.write_u32::<LittleEndian>(value.len() as u32)?;
    write!(output, "{}", value)?;

    Ok(())
}

/// Applies the integer transformation generally used in property data in the
/// Roblox binary format.
pub fn transform_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

/// The inverse of `transform_i32`.
pub fn untransform_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}
