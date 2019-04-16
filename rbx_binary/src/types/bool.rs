use std::{
    io::{self, Read, Write},
};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::core::BinaryType;

pub struct BoolType;

impl BinaryType for BoolType {
    type WriteItem = bool;
    type ReadItem = bool;

    fn read_one<R: Read>(source: &mut R) -> io::Result<bool> {
        Ok(source.read_u8()? != 0)
    }

    fn write_one<W: Write>(output: &mut W, value: &bool) -> io::Result<()> {
        output.write_u8(*value as u8)
    }
}