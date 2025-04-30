// Terrain has two binary formats, being the material colors and smooth grid blobs.
// The smooth grid spec can be found at docs/smooth-grid.md from the root directory of this project.
// The material colors spec can be found at docs/binary-strings.md from the root directory of this project.

mod material_colors;
mod smooth_grid;

pub use self::material_colors::*;
pub use self::smooth_grid::*;

use std::fmt;
use std::io::{Read, Result, Write};
use std::mem;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TerrainMaterials {
    Air,
    Water,
    Grass,
    Slate,
    Concrete,
    Brick,
    Sand,
    WoodPlanks,
    Rock,
    Glacier,
    Snow,
    Sandstone,
    Mud,
    Basalt,
    Ground,
    CrackedLava,
    Asphalt,
    Cobblestone,
    Ice,
    LeafyGrass,
    Salt,
    Limestone,
    Pavement,
}

impl fmt::Display for TerrainMaterials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Takes `values` and writes it as a blob of data with each value
/// interleaved by `N` bytes.
///
/// This function allocates `N * values.len()` bytes before writing.
pub(crate) fn write_interleaved_bytes<W, const N: usize>(
    writer: &mut W,
    values: &[[u8; N]],
) -> Result<()>
where
    W: Write,
{
    let len = values.len();
    let mut blob = vec![0; len * N];
    for (i, bytes) in values.iter().enumerate() {
        for (j, byte) in bytes.iter().enumerate() {
            blob[i + len * j] = *byte;
        }
    }
    writer.write_all(&blob)?;

    Ok(())
}

/// Writes all items from `values` into the buffer as a blob of interleaved
/// bytes. Transformation is applied to the values as they're written.
pub(crate) fn write_interleaved_i32_array<W, I>(writer: &mut W, values: I) -> Result<()>
where
    W: Write,
    I: Iterator<Item = i32>,
{
    let values: Vec<_> = values.map(|v| v.to_be_bytes()).collect();
    write_interleaved_bytes(writer, &values)
}

/// Fills `output` with blocks of `N` bytes from the buffer,
/// deinterleaving them in the process.
///
/// This function allocates `N * output.len()` bytes before reading.
pub(crate) fn read_interleaved_bytes<R, const N: usize>(
    reader: &mut R,
    output: &mut [[u8; N]],
) -> Result<()>
where
    R: Read,
{
    let len = output.len();
    let mut buffer = vec![0; len * N];
    reader.read_exact(&mut buffer)?;

    for (i, array) in output.iter_mut().enumerate() {
        for (j, byte) in array.iter_mut().enumerate() {
            *byte = buffer[i + len * j];
        }
    }

    Ok(())
}

/// Fills `output` with big-endian `i32` values read from the buffer.
/// These values are untransformed while being read.
pub(crate) fn read_interleaved_i32_array<R>(reader: &mut R, output: &mut [i32]) -> Result<()>
where
    R: Read,
{
    let mut read = vec![[0; mem::size_of::<i32>()]; output.len()];
    read_interleaved_bytes(reader, &mut read)?;

    for (chunk, out) in read.into_iter().zip(output) {
        *out = i32::from_be_bytes(chunk);
    }

    Ok(())
}
