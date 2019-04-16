use std::io::{self, Read, Write};

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

pub trait BinaryType {
    type WriteItem: ?Sized + 'static;
    type ReadItem: 'static;

    fn read_one<R: Read>(source: &mut R) -> io::Result<Self::ReadItem> {
        let items = Self::read_many(source, 1)?;
        Ok(items.into_iter().next().unwrap())
    }

    fn read_many<R: Read>(source: &mut R, count: usize) -> io::Result<Vec<Self::ReadItem>> {
        let mut output = Vec::with_capacity(count);

        for _ in 0..count {
            output.push(Self::read_one(source)?);
        }

        Ok(output)
    }

    fn write_one<W: Write>(output: &mut W, value: &Self::WriteItem) -> io::Result<()> {
        Self::write_many(output, &[value])
    }

    fn write_many<W: Write>(output: &mut W, values: &[&Self::WriteItem]) -> io::Result<()> {
        for value in values {
            Self::write_one(output, value)?;
        }

        Ok(())
    }
}