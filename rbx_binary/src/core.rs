use std::io::{self, Read, Write};

use rbx_dom_weak::RbxValue;

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

pub trait BinaryType<T: ?Sized> {
    fn read_binary<R: Read>(
        source: &mut R,
    ) -> io::Result<RbxValue>;

    fn write_binary<W: Write>(
        output: &mut W,
        value: &T,
    ) -> io::Result<()>;

	fn read_array<R: Read>(
		source: &mut R,
		count: usize,
	) -> io::Result<Vec<RbxValue>> {
		let mut result = Vec::new();

		for _ in 0..count {
			result.push(Self::read_binary(source)?);
		}

		Ok(result)
	}

	fn write_array<W: Write>(
		output: &mut W,
		values: &[&T]
	) -> io::Result<()> {
		for value in values {
			Self::write_binary(output, value)?;
		}

		Ok(())
	}
}
