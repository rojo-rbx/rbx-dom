use std::{
    fmt,
    io::{self, Read, Write},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// Represents one chunk from a binary model file.
#[derive(Debug)]
pub struct Chunk {
    pub name: [u8; 4],
    pub data: Vec<u8>,
}

impl Chunk {
    /// Reads and decodes a `Chunk` from the given reader.
    pub fn decode<R: Read>(mut reader: R) -> io::Result<Chunk> {
        let header = decode_chunk_header(&mut reader)?;

        log::trace!("{}", header);

        let data = if header.compressed_len == 0 {
            let mut data = Vec::with_capacity(header.len as usize);
            reader.take(header.len as u64).read_to_end(&mut data)?;
            data
        } else {
            let mut compressed_data = Vec::with_capacity(header.compressed_len as usize);
            reader
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
}

/// The compression format of a chunk in the binary model format.
#[derive(Debug, Clone, Copy)]
pub enum ChunkCompression {
    /// The contents of the chunk should be LZ4 compressed.
    Compressed,

    /// The contents of the chunk should be uncompressed.
    Uncompressed,
}

/// Holds a chunk that is currently being written.
///
/// This type intended to be written into via io::Write and then dumped into the
/// output stream all at once. It handles compression and chunk header output
/// automatically.
#[must_use]
pub struct ChunkBuilder {
    chunk_name: &'static [u8],
    compression: ChunkCompression,
    buffer: Vec<u8>,
}

impl ChunkBuilder {
    /// Creates a new `ChunkBuilder` with the given name and compression
    /// setting.
    pub fn new(chunk_name: &'static [u8], compression: ChunkCompression) -> Self {
        ChunkBuilder {
            chunk_name,
            compression,
            buffer: Vec::new(),
        }
    }

    /// Consume the chunk and write it to the given writer.
    pub fn dump<W: Write>(self, mut writer: W) -> io::Result<()> {
        writer.write_all(self.chunk_name)?;

        match self.compression {
            ChunkCompression::Compressed => {
                let compressed = lz4::block::compress(&self.buffer, None, false)?;

                writer.write_u32::<LittleEndian>(compressed.len() as u32)?;
                writer.write_u32::<LittleEndian>(self.buffer.len() as u32)?;
                writer.write_u32::<LittleEndian>(0)?;

                writer.write_all(&compressed)?;
            }
            ChunkCompression::Uncompressed => {
                writer.write_u32::<LittleEndian>(0)?;
                writer.write_u32::<LittleEndian>(self.buffer.len() as u32)?;
                writer.write_u32::<LittleEndian>(0)?;

                writer.write_all(&self.buffer)?;
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

    if reserved != 0 {
        panic!(
            "Chunk reserved space was not zero, it was {}. This chunk may be malformed.",
            reserved
        );
    }

    Ok(ChunkHeader {
        name,
        compressed_len,
        len,
        reserved,
    })
}
