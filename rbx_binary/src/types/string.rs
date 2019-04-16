use std::{
    io::{self, Read, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::core::BinaryType;

pub struct StringType;

impl BinaryType for StringType {
    type ReadItem = String;
    type WriteItem = str;

    fn read_one<R: Read>(source: &mut R) -> io::Result<String> {
        let length = source.read_u32::<LittleEndian>()?;

        let mut value = String::with_capacity(length as usize);
        source.take(length as u64).read_to_string(&mut value)?;

        Ok(value)
    }

    fn write_one<W: Write>(output: &mut W, value: &str) -> io::Result<()> {
        output.write_u32::<LittleEndian>(value.len() as u32)?;
        write!(output, "{}", value)
    }
}