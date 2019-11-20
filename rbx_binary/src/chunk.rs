use std::{
    fmt,
    io::{self, Read, Write},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// The compression format of a chunk in the binary model format.
#[derive(Debug, Clone, Copy)]
pub enum Compression {
    Compressed,
    Uncompressed,
}

/// Holds a chunk that is currently being written.
///
/// This type intended to be written into via io::Write and then dumped into the
/// output stream all at once. It handles compression and chunk header output
/// automatically.
pub struct ChunkBuilder {
    chunk_name: &'static [u8],
    compression: Compression,
    buffer: Vec<u8>,
}

impl ChunkBuilder {
    pub fn new(chunk_name: &'static [u8], compression: Compression) -> Self {
        ChunkBuilder {
            chunk_name,
            compression,
            buffer: Vec::new(),
        }
    }

    pub fn dump<W: Write>(self, output: &mut W) -> io::Result<()> {
        output.write_all(self.chunk_name)?;

        match self.compression {
            Compression::Compressed => {
                let compressed = lz4::block::compress(&self.buffer, None, false)?;

                output.write_u32::<LittleEndian>(compressed.len() as u32)?;
                output.write_u32::<LittleEndian>(self.buffer.len() as u32)?;
                output.write_u32::<LittleEndian>(0)?;

                output.write_all(&compressed)?;
            }
            Compression::Uncompressed => {
                output.write_u32::<LittleEndian>(0)?;
                output.write_u32::<LittleEndian>(self.buffer.len() as u32)?;
                output.write_u32::<LittleEndian>(0)?;

                output.write_all(&self.buffer)?;
            }
        }

        Ok(())
    }
}

impl Write for ChunkBuilder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub name: [u8; 4],
    pub data: Vec<u8>,
}

pub fn decode_chunk<R: Read>(source: &mut R) -> io::Result<Chunk> {
    let header = decode_chunk_header(source)?;

    log::trace!("{}", header);

    let data = if header.compressed_len == 0 {
        let mut data = Vec::with_capacity(header.len as usize);
        source.take(header.len as u64).read_to_end(&mut data)?;
        data
    } else {
        let mut compressed_data = Vec::with_capacity(header.compressed_len as usize);
        source
            .take(header.compressed_len as u64)
            .read_to_end(&mut compressed_data)?;

        lz4::block::decompress(&compressed_data, Some(header.len as i32))?
    };

    assert_eq!(data.len(), header.len as usize);

    Ok(Chunk {
        name: header.name,
        data,
    })
}

#[derive(Debug)]
struct ChunkHeader {
    /// 4-byte short name for the chunk, like "INST" or "PRNT"
    name: [u8; 4],

    /// The length of the chunk's compressed data. For uncompressed chunks, this
    /// is always zero.
    compressed_len: u32,

    /// The length that the chunk's data will have when decompressed. For
    /// uncompressed chunks, this is their length as-is.
    len: u32,

    /// Always zero.
    reserved: u32,
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        let name = if let Ok(name) = str::from_utf8(&self.name) {
            name.to_owned()
        } else {
            format!("{:?}", self.name)
        };

        write!(
            output,
            "Chunk \"{}\" (compressed: {}, len: {}, reserved: {})",
            name, self.compressed_len, self.len, self.reserved
        )
    }
}

fn decode_chunk_header<R: Read>(source: &mut R) -> io::Result<ChunkHeader> {
    let mut name = [0; 4];
    source.read_exact(&mut name)?;

    let compressed_len = source.read_u32::<LittleEndian>()?;
    let len = source.read_u32::<LittleEndian>()?;
    let reserved = source.read_u32::<LittleEndian>()?;

    Ok(ChunkHeader {
        name,
        compressed_len,
        len,
        reserved,
    })
}
