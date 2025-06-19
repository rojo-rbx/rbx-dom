use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    io::{self, Read},
};

use thiserror::Error;

use crate::Vector3;

use crate::Error as CrateError;

use super::{read_interleaved_i32_array, write_interleaved_i32_array, TerrainMaterials};

/// Expected binary version, written to the blob.
const BINARY_VERSION: u8 = 1;

/// Expected binary chunk size (power of 2). If the chunk size is different,
/// Roblox will convert it. The behavior of this conversion is unknown.
const BINARY_CHUNK_SIZE: u8 = 5;

/// Size of a chunk. Chunks are cubes, so this is the length/width/height.
const CHUNK_SIZE: i32 = 2i32.pow(BINARY_CHUNK_SIZE as u32);

/// Coordinates of a chunk or a voxel. For internal use.
// Can't use Vector3int16; we need a 32 bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TerrainVec {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl TerrainVec {
    /// Naively converts the components to be `i32`. This will always round down due to truncation.
    pub fn from_vec3(i: Vector3) -> Self {
        Self {
            x: i.x as i32,
            y: i.y as i32,
            z: i.z as i32,
        }
    }

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

/// Coordinates of a `Voxel` inside of a `Chunk`, which is a grid of 4 units in world space.
/// This is inside of a grid of 32^3 voxels per chunk.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct VoxelCoordinates(TerrainVec);

impl VoxelCoordinates {
    /// Constructs a new `VoxelCoordinates` object.
    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        const VOXEL_MAX: i32 = CHUNK_SIZE - 1;
        Self(TerrainVec::new(
            x.clamp(0, VOXEL_MAX),
            y.clamp(0, VOXEL_MAX),
            z.clamp(0, VOXEL_MAX),
        ))
    }

    /// Constructs a new `VoxelCoordinates` object from a Vector3.
    #[inline]
    pub fn from_vec3(i: Vector3) -> Self {
        Self(TerrainVec::from_vec3(i))
    }
}

/// Coordinates of a `Chunk` in chunk space, which is a grid of 128 units in world space.
/// Relevant for usage with a `Terrain` object. Inside a grid of 524,288^3 chunks per world.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct ChunkCoordinates(TerrainVec);

impl ChunkCoordinates {
    /// Constructs a new `ChunkCoordinates` object.
    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        // Terrain isn't accepted by the engine with voxels past 2^23.
        const CHUNK_MAX: i32 = 2i32.pow(23) / CHUNK_SIZE;
        Self(TerrainVec::new(
            x.clamp(-CHUNK_MAX, CHUNK_MAX),
            y.clamp(-CHUNK_MAX, CHUNK_MAX),
            z.clamp(-CHUNK_MAX, CHUNK_MAX),
        ))
    }

    /// Constructs a new `ChunkCoordinates` object from a Vector3.
    #[inline]
    pub fn from_vec3(i: Vector3) -> Self {
        Self(TerrainVec::from_vec3(i))
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum TerrainGridMaterial {
    #[default]
    Air = 0x00,
    Water = 0x01,
    Grass = 0x02,
    Slate = 0x03,
    Concrete = 0x04,
    Brick = 0x05,
    Sand = 0x06,
    WoodPlanks = 0x07,
    Rock = 0x08,
    Glacier = 0x09,
    Snow = 0x0A,
    Sandstone = 0x0B,
    Mud = 0x0C,
    Basalt = 0x0D,
    Ground = 0x0E,
    CrackedLava = 0x0F,
    Asphalt = 0x10,
    Cobblestone = 0x11,
    Ice = 0x12,
    LeafyGrass = 0x13,
    Salt = 0x14,
    Limestone = 0x15,
    Pavement = 0x16,
}

impl From<TerrainMaterials> for TerrainGridMaterial {
    fn from(value: TerrainMaterials) -> Self {
        use TerrainGridMaterial::*;

        match value {
            TerrainMaterials::Air => Air,
            TerrainMaterials::Water => Water,
            TerrainMaterials::Grass => Grass,
            TerrainMaterials::Slate => Slate,
            TerrainMaterials::Concrete => Concrete,
            TerrainMaterials::Brick => Brick,
            TerrainMaterials::Sand => Sand,
            TerrainMaterials::WoodPlanks => WoodPlanks,
            TerrainMaterials::Rock => Rock,
            TerrainMaterials::Glacier => Glacier,
            TerrainMaterials::Snow => Snow,
            TerrainMaterials::Sandstone => Sandstone,
            TerrainMaterials::Mud => Mud,
            TerrainMaterials::Basalt => Basalt,
            TerrainMaterials::Ground => Ground,
            TerrainMaterials::CrackedLava => CrackedLava,
            TerrainMaterials::Asphalt => Asphalt,
            TerrainMaterials::Cobblestone => Cobblestone,
            TerrainMaterials::Ice => Ice,
            TerrainMaterials::LeafyGrass => LeafyGrass,
            TerrainMaterials::Salt => Salt,
            TerrainMaterials::Limestone => Limestone,
            TerrainMaterials::Pavement => Pavement,
        }
    }
}

impl TryFrom<u8> for TerrainGridMaterial {
    type Error = CrateError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use TerrainGridMaterial::*;

        Ok(match value {
            0x00 => Air,
            0x01 => Water,
            0x02 => Grass,
            0x03 => Slate,
            0x04 => Concrete,
            0x05 => Brick,
            0x06 => Sand,
            0x07 => WoodPlanks,
            0x08 => Rock,
            0x09 => Glacier,
            0x0A => Snow,
            0x0B => Sandstone,
            0x0C => Mud,
            0x0D => Basalt,
            0x0E => Ground,
            0x0F => CrackedLava,
            0x10 => Asphalt,
            0x11 => Cobblestone,
            0x12 => Ice,
            0x13 => LeafyGrass,
            0x14 => Salt,
            0x15 => Limestone,
            0x16 => Pavement,
            _ => return Err(SmoothGridError::UnknownMaterial(value).into()),
        })
    }
}

/// An error that can occur when deserializing or working with SmoothGrid.
#[derive(Debug, Error)]
pub(crate) enum SmoothGridError {
    /// The argument provided to `try_from<u8>` did not correspond to a known
    /// TerrainGridMaterial.
    #[error("cannot convert `{0}` into TerrainGridMaterial")]
    UnknownMaterial(u8),

    #[error("expected file header with version {BINARY_VERSION} and size {BINARY_CHUNK_SIZE}, received version {0} and size {1}")]
    InvalidHeader(u8, u8),

    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },
}

/// A container for a voxel of terrain, used in the `Chunk` object.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Voxel {
    solid_occupancy: f32,
    water_occupancy: f32,
    material: TerrainGridMaterial,
}

bitflags::bitflags! {
    struct VoxelFlags: u8 {
        const HAS_OCCUPANCY = 0b01000000;
        const HAS_COUNT = 0b10000000;
    }
}

impl Voxel {
    /// Constructs a new `Voxel` with a material and occupancy percentage.
    /// Equivalent to data writeable from Roblox's `Terrain:WriteVoxels`.
    /// Occupancy values are between `0.0` and `1.0`, as a percentage of the voxel.
    pub fn new(material: TerrainMaterials, solid_occupancy: f32) -> Self {
        let mut voxel = Self {
            material: material.into(),
            ..Default::default()
        };
        voxel.set_occupancy(solid_occupancy, 0.0);

        voxel
    }

    /// Constructs a new `Voxel` with a material, solid occupancy, and water
    /// occupancy percentage.
    /// Equivalent to data writeable from Roblox's `Terrain:WriteVoxelChannels`.
    /// Occupancy values are between `0.0` and `1.0`, as a percentage of the voxel.
    pub fn new_with_water(
        material: TerrainMaterials,
        solid_occupancy: f32,
        water_occupancy: f32,
    ) -> Self {
        let mut voxel = Self {
            material: material.into(),
            ..Default::default()
        };
        voxel.set_occupancy(solid_occupancy, water_occupancy);

        voxel
    }

    fn get_encode_data(&self) -> (u8, u8) {
        (
            (self.solid_occupancy * 255.0) as u8,
            (self.water_occupancy * 255.0) as u8,
        )
    }

    fn encode_run_length(&self, count: u16) -> Vec<u8> {
        assert!(
            (1..=256).contains(&count),
            "Invalid voxel RLE count: {}",
            count
        );

        let (solid_occupancy, water_occupancy) = self.get_encode_data();
        let mut flag = self.material as u8;

        // The first value is a placeholder that'll be replaced with the `flag` later
        let mut to_write: Vec<u8> = vec![0x00];

        if solid_occupancy != 0xFF && solid_occupancy != 0x00 {
            // Should we store the solid occupancy value?
            flag |= VoxelFlags::HAS_OCCUPANCY.bits();
            to_write.push(solid_occupancy);
        }
        if count > 1 || water_occupancy != 0 {
            // Should we store the count (amount of voxels this run length) value?
            flag |= VoxelFlags::HAS_COUNT.bits();
            if water_occupancy == 0 {
                to_write.push((count - 1) as u8);
            } else {
                to_write.push(0);
                to_write.push(water_occupancy);
            }
        }
        to_write[0] = flag;

        if water_occupancy != 0x00 && count > 1 {
            // Shorelines uses a new water occupancy value in the voxel data. Because of this,
            // Roblox uses a hack to avoid having to reduce their 6 bits of material ID freedom
            // by writing voxels with a count bit set to 1 and no count. This means we have to write
            // all voxels in the run length manually.
            let len = to_write.len();
            return to_write
                .into_iter()
                .cycle()
                .take(len * count as usize)
                .collect();
        }
        to_write
    }

    /// Sets occupancy data for a `Voxel`. Water occupancy is from the
    /// Shorelines feature. Occupancy values are between `0.0` and `1.0`,
    /// as a percentage of the voxel.
    pub fn set_occupancy(&mut self, solid_occupancy: f32, water_occupancy: f32) {
        let solid_occupancy = solid_occupancy.clamp(0.0, 1.0);
        let water_occupancy = water_occupancy.clamp(0.0, 1.0);

        // If we have nothing in there, it should just be air.
        if solid_occupancy == 0.0 && water_occupancy == 0.0 {
            self.solid_occupancy = 1.0;
            self.water_occupancy = 0.0;
            self.material = TerrainGridMaterial::Air;
            return;
        }

        // We should encode water as a normal, non-shorelines voxel if there's no solids.
        if (solid_occupancy == 0.0 || self.material == TerrainGridMaterial::Air)
            && water_occupancy > 0.0
        {
            self.solid_occupancy = water_occupancy;
            self.water_occupancy = 0.0;
            self.material = TerrainGridMaterial::Water;
            return;
        }

        self.solid_occupancy = solid_occupancy;

        // Full with a solid (non-air) material? We can't have any water.
        if solid_occupancy == 1.0 {
            self.water_occupancy = 0.0
        } else {
            self.water_occupancy = water_occupancy;
        }
    }

    pub fn set_material(&mut self, material: TerrainMaterials) {
        self.material = material.into();

        // Occupancy determination depends on material.
        self.set_occupancy(self.solid_occupancy, self.water_occupancy)
    }
}

/// A container for a chunk of terrain, used in the `Terrain` object.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chunk {
    grid: HashMap<VoxelCoordinates, Voxel>,
    /// For all empty voxels in the chunk, we will write this material
    /// at 100% occupancy. Defaults to `TerrainGridMaterial::Air`.
    base_material: TerrainGridMaterial,
}

impl Chunk {
    /// Constructs a new `Chunk` with a base material of `TerrainGridMaterial::Air`.
    #[inline]
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            base_material: TerrainGridMaterial::Air,
        }
    }

    /// Constructs a new `Chunk` with a user-provided base material.
    #[inline]
    pub fn new_with_base(base_material: TerrainMaterials) -> Self {
        Self {
            grid: HashMap::new(),
            base_material: base_material.into(),
        }
    }

    /// Changes the base material of a `Chunk` to a user-provided base material.
    #[inline]
    pub fn set_base(&mut self, base_material: TerrainMaterials) {
        self.base_material = base_material.into();
    }

    /// Finds a `Voxel` at the given position in this `Chunk`,
    /// returning a reference to it if it exists.
    #[inline]
    pub fn get_voxel(&self, position: &VoxelCoordinates) -> Option<&Voxel> {
        self.grid.get(position)
    }

    /// Finds a `Voxel` at the given position in this `Chunk`,
    /// returning a mutable reference to it if it exists.
    #[inline]
    pub fn get_voxel_mut(&mut self, position: &VoxelCoordinates) -> Option<&mut Voxel> {
        self.grid.get_mut(position)
    }

    /// Writes (or overwrites) a `Chunk` at the given position to this `Terrain`.
    #[inline]
    pub fn write_voxel(&mut self, position: &VoxelCoordinates, voxel: Voxel) {
        self.grid.insert(*position, voxel);
    }

    fn encode(&self) -> Vec<u8> {
        // ~256 bytes if all voxels are air/base mat with maximum count. Double it
        let mut data = Vec::with_capacity(512);

        let base_voxel = Voxel {
            solid_occupancy: 1.0,
            water_occupancy: 0.0,
            material: self.base_material,
        };

        let mut pos_cursor = VoxelCoordinates::default();
        let mut run_length_cursor: u16 = 0;
        let mut run_length_voxel = &base_voxel;
        for y in 0..CHUNK_SIZE {
            pos_cursor.0.y = y;
            for z in 0..CHUNK_SIZE {
                pos_cursor.0.z = z;
                for x in 0..CHUNK_SIZE {
                    pos_cursor.0.x = x;

                    let grabbed_voxel = self.grid.get(&pos_cursor).unwrap_or(&base_voxel);

                    if run_length_cursor == 0 {
                        // We don't add 1 here, next if statement does it.
                        run_length_voxel = grabbed_voxel;
                    }
                    if grabbed_voxel == run_length_voxel {
                        if run_length_cursor < 0xFF {
                            run_length_cursor += 1;
                            continue;
                        } else {
                            // Properly reset the run-length if we hit the max.
                            data.extend(grabbed_voxel.encode_run_length(run_length_cursor + 1));
                            run_length_cursor = 0;
                            continue;
                        }
                    }

                    data.extend(run_length_voxel.encode_run_length(run_length_cursor));
                    run_length_cursor = 1;
                    run_length_voxel = grabbed_voxel;
                }
            }
        }

        // We might have a bit of leftovers after that loop.
        if run_length_cursor > 0 {
            data.extend(run_length_voxel.encode_run_length(run_length_cursor));
        }
        data
    }

    /// Decodes a `Chunk` from a binary blob. The blob must be the same format used
    /// by `encode` and Roblox.
    pub fn decode(mut buffer: &[u8]) -> Result<Self, CrateError> {
        let mut chunk = Chunk::new();

        let mut voxel_count = 0;
        while voxel_count < CHUNK_SIZE.pow(3) {
            let mut current_voxel_buffer = [0u8];
            if let Err(e) = buffer.read_exact(&mut current_voxel_buffer) {
                return Err(SmoothGridError::from(e).into());
            }

            let voxel_flag = VoxelFlags::from_bits_truncate(current_voxel_buffer[0]);

            let mut occupancy: Option<f32> = None;
            let mut count: Option<u8> = None;
            let mut water_occupancy: Option<f32> = None;
            let material = TerrainGridMaterial::try_from(current_voxel_buffer[0])?;

            if voxel_flag.contains(VoxelFlags::HAS_OCCUPANCY) {
                let mut occupancy_buffer = [0u8];
                if let Err(e) = buffer.read_exact(&mut occupancy_buffer) {
                    return Err(SmoothGridError::from(e).into());
                }

                occupancy = Some(occupancy_buffer[0] as f32 / 255.0);
            }
            if voxel_flag.contains(VoxelFlags::HAS_COUNT) {
                let mut count_buffer = [0u8];
                if let Err(e) = buffer.read_exact(&mut count_buffer) {
                    return Err(SmoothGridError::from(e).into());
                }

                if count_buffer[0] == 0 {
                    let mut water_occupancy_buffer = [0u8];
                    if let Err(e) = buffer.read_exact(&mut water_occupancy_buffer) {
                        return Err(SmoothGridError::from(e).into());
                    }

                    water_occupancy = Some(water_occupancy_buffer[0] as f32 / 255.0)
                } else {
                    count = Some(count_buffer[0]);
                }
            }

            if let Some(voxel_amount) = count {
                for _ in 0..voxel_amount {
                    chunk.write_voxel(
                        &VoxelCoordinates::new(
                            voxel_count / (CHUNK_SIZE.pow(2)),
                            voxel_count / CHUNK_SIZE,
                            voxel_count % CHUNK_SIZE,
                        ),
                        Voxel {
                            material,
                            solid_occupancy: occupancy.unwrap_or(0.0),
                            water_occupancy: 0.0,
                        },
                    );
                    voxel_count += 1;
                }
            } else {
                chunk.write_voxel(
                    &VoxelCoordinates::new(
                        voxel_count / (CHUNK_SIZE.pow(2)),
                        voxel_count / CHUNK_SIZE,
                        voxel_count % CHUNK_SIZE,
                    ),
                    Voxel {
                        material,
                        solid_occupancy: occupancy.unwrap_or(0.0),
                        water_occupancy: water_occupancy.unwrap_or(0.0),
                    },
                );
                voxel_count += 1;
            }
        }

        Ok(chunk)
    }
}

/// A container allowing the modification, encoding, and decoding of the
/// `SmoothGrid` data used by Roblox's `Terrain` object.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct SmoothGrid {
    world: BTreeMap<ChunkCoordinates, Chunk>,
}

impl SmoothGrid {
    /// Constructs a new `Terrain` with no chunks.
    #[inline]
    pub fn new() -> Self {
        Self {
            world: BTreeMap::new(),
        }
    }

    /// Finds a `Chunk` at the given position in this `Terrain`,
    /// returning a reference to it if it exists.
    #[inline]
    pub fn get_chunk(&self, position: &ChunkCoordinates) -> Option<&Chunk> {
        self.world.get(position)
    }

    /// Finds a `Chunk` at the given position in this `Terrain`,
    /// returning a mutable reference to it if it exists.
    #[inline]
    pub fn get_chunk_mut(&mut self, position: &ChunkCoordinates) -> Option<&mut Chunk> {
        self.world.get_mut(position)
    }

    /// Writes (or overwrites) a `Chunk` at the given position in this `Terrain`.
    #[inline]
    pub fn write_chunk(&mut self, position: &ChunkCoordinates, chunk: Chunk) {
        self.world.insert(*position, chunk);
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.world.len() * 512);
        data.extend([0x01, CHUNK_SIZE.ilog2() as u8]);

        let mut chunk_cursor = None;
        for (position, chunk) in &self.world {
            let cursor = chunk_cursor.unwrap_or(position);
            let axes = [
                position.0.x - cursor.0.x,
                position.0.y - cursor.0.y,
                position.0.z - cursor.0.z,
            ];

            write_interleaved_i32_array(&mut data, axes.iter().copied()).unwrap();

            data.extend(chunk.encode());
            chunk_cursor = Some(position);
        }

        data
    }

    /// Decodes a `SmoothGrid` from a binary blob. The blob must be the same format used
    /// by `encode` and Roblox.
    pub fn decode(mut buffer: &[u8]) -> Result<Self, CrateError> {
        let mut header = [0u8, 0u8];
        if let Err(e) = buffer.read_exact(&mut header) {
            return Err(SmoothGridError::from(e).into());
        }

        let [version, chunk_size] = header;
        if version != BINARY_VERSION || chunk_size != BINARY_CHUNK_SIZE {
            return Err(SmoothGridError::InvalidHeader(version, chunk_size).into());
        }

        let mut world = Self::new();
        let mut offset_buffer = [0; 3];
        let mut cursor_buffer = [0; 3];
        while read_interleaved_i32_array(&mut buffer, &mut offset_buffer).is_ok() {
            let true_position_buffer = [
                cursor_buffer[0] + offset_buffer[0],
                cursor_buffer[1] + offset_buffer[1],
                cursor_buffer[2] + offset_buffer[2],
            ];
            cursor_buffer.copy_from_slice(&true_position_buffer[..]);

            let true_position = ChunkCoordinates::new(
                true_position_buffer[0],
                true_position_buffer[1],
                true_position_buffer[2],
            );
            world.write_chunk(&true_position, Chunk::decode(buffer)?);
        }

        Ok(world)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_default() {
        let mut terr = SmoothGrid::new();
        let mut chunk = Chunk::new_with_base(TerrainMaterials::Air);

        let mut voxel = Voxel::new_with_water(TerrainMaterials::Water, 1.0, 0.5);
        chunk.write_voxel(&VoxelCoordinates::new(0, 0, 0), voxel);
        voxel.set_material(TerrainMaterials::Pavement);
        chunk.write_voxel(&VoxelCoordinates::new(1, 0, 0), voxel);

        terr.write_chunk(&ChunkCoordinates::default(), chunk.clone());
        terr.write_chunk(&ChunkCoordinates::new(1, 0, 0), chunk.clone());

        let encoded = base64::encode(terr.encode());
        assert_eq!(encoded, "AQUAAAAAAAAAAAAAAAABFoD/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP0AAAAAAAAAAAABAAABFoD/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP+A/4D/gP0=")
    }
}
