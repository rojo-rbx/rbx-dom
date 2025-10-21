// Terrain has two binary formats, being the material colors and smooth grid blobs.
// The smooth grid spec can be found at docs/smooth-grid.md from the root directory of this project.
// The material colors spec can be found at docs/binary-strings.md from the root directory of this project.

mod material_colors;
mod serialization;
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

#[cfg(test)]
mod test {
    use super::*;

    /// This is a confirmed valid snapshot of the testing data created by
    /// the encoder in the `test_binary_encode` test, and is in the snapshot.
    const TERRAIN_BASE64: &str = "AQUAAAAAAAAAAAAAAAABFoD/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP0AAAAAAAAAAAABAAABFoD/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP0=";

    #[test]
    fn test_grid_encode() {
        let mut terr = SmoothGrid::new();
        let mut chunk = Chunk::new_with_base(TerrainMaterials::Air);

        let mut voxel = Voxel::new_with_water(TerrainMaterials::Water, 1.0, 0.5);
        chunk.write_voxel(&VoxelCoordinates::new(0, 0, 0), voxel);
        voxel.set_material(TerrainMaterials::Pavement);
        chunk.write_voxel(&VoxelCoordinates::new(1, 0, 0), voxel);

        terr.write_chunk(&ChunkCoordinates::default(), chunk.clone());
        terr.write_chunk(&ChunkCoordinates::new(1, 0, 0), chunk.clone());

        let encoded_grid = base64::encode(terr.encode());
        assert_eq!(encoded_grid, TERRAIN_BASE64)
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_round_trip_grid() {
        let grid_bytes = base64::decode(TERRAIN_BASE64).expect("bad base64 for terrain");
        let grid = SmoothGrid::decode(grid_bytes.as_slice()).expect("couldn't deserialize terrain");

        let encoded = grid.encode();

        // Because the serde implementation for SmoothGrid is naive, and solely
        // calls into the actual encoder, this will test the encoder.
        insta::assert_yaml_snapshot!(grid);
    }
}
