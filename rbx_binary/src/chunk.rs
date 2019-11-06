use std::{
    borrow::Cow,
    fmt,
    io::{self, Cursor, Read, Write},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub enum Compression {
    Compressed,
    Uncompressed,
}

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
pub struct ChunkHeader {
    pub name: [u8; 4],
    pub compressed_len: u32,
    pub len: u32,
    pub reserved: u32,
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        let name = if let Ok(name) = str::from_utf8(&self.name) {
            Cow::Borrowed(name)
        } else {
            Cow::Owned(format!("{:?}", self.name))
        };

        write!(
            output,
            "Chunk \"{}\" (compressed: {}, len: {}, reserved: {})",
            name, self.compressed_len, self.len, self.reserved
        )
    }
}

pub fn encode_chunk<W: Write, F>(
    output: &mut W,
    chunk_name: &[u8],
    compression: Compression,
    body: F,
) -> io::Result<()>
where
    F: Fn(Cursor<&mut Vec<u8>>) -> io::Result<()>,
{
    output.write_all(chunk_name)?;

    let mut buffer = Vec::new();
    body(Cursor::new(&mut buffer))?;

    match compression {
        Compression::Compressed => {
            let compressed = lz4::block::compress(&buffer, None, false)?;

            output.write_u32::<LittleEndian>(compressed.len() as u32)?;
            output.write_u32::<LittleEndian>(buffer.len() as u32)?;
            output.write_u32::<LittleEndian>(0)?;

            output.write_all(&compressed)?;
        }
        Compression::Uncompressed => {
            output.write_u32::<LittleEndian>(0)?;
            output.write_u32::<LittleEndian>(buffer.len() as u32)?;
            output.write_u32::<LittleEndian>(0)?;

            output.write_all(&buffer)?;
        }
    }

    Ok(())
}

pub fn decode_chunk<R: Read>(source: &mut R, output: &mut Vec<u8>) -> io::Result<ChunkHeader> {
    let header = decode_chunk_header(source)?;

    log::trace!("{}", header);

    if header.compressed_len == 0 {
        source.take(header.len as u64).read_to_end(output)?;
    } else {
        let mut compressed_data = Vec::new();
        source
            .take(header.compressed_len as u64)
            .read_to_end(&mut compressed_data)?;

        let data = lz4::block::decompress(&compressed_data, Some(header.len as i32))?;
        output.extend_from_slice(&data);
    }

    assert_eq!(output.len(), header.len as usize);

    Ok(header)
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
