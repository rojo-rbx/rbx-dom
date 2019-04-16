use std::io::{self, Read, Write};

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
