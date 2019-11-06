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

pub trait RbxReadExt: Read {
    fn read_string(&mut self) -> io::Result<String> {
        let length = self.read_u32::<LittleEndian>()?;

        let mut value = String::with_capacity(length as usize);
        self.take(length as u64).read_to_string(&mut value)?;

        Ok(value)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }
}

impl<R> RbxReadExt for R where R: Read {}

pub trait RbxWriteExt: Write {
    fn write_string(&mut self, value: &str) -> io::Result<()> {
        self.write_u32::<LittleEndian>(value.len() as u32)?;
        write!(self, "{}", value)?;

        Ok(())
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(value as u8)
    }
}

impl<W> RbxWriteExt for W where W: Write {}

/// Applies the integer transformation generally used in property data in the
/// Roblox binary format.
pub fn transform_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

/// The inverse of `transform_i32`.
pub fn untransform_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}
