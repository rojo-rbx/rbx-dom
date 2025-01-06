use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
};

use thiserror::Error;

use crate::Vector3;

use crate::Error as CrateError;

use super::TerrainMaterials;

/// Size of a chunk. Chunks are cubes, so this is the length/width/height.
const CHUNK_SIZE: i32 = 2i32.pow(5);

/// Coordinates of a chunk or a voxel. For internal use.
// Can't use Vector3int16; we need a 32 bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TerrainVec {
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
}

/// A container for a voxel of terrain, used in the `Chunk` object.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Voxel {
    solid_occupancy: f32,
    water_occupancy: f32,
    material: TerrainGridMaterial,
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
            flag |= 0b01000000;
            to_write.push(solid_occupancy);
        }
        if count > 1 || water_occupancy != 0 {
            // Should we store the count (amount of voxels this run length) value?
            flag |= 0b10000000;
            if water_occupancy == 0 {
                to_write.push((count - 1) as u8);
            } else {
                to_write.push(0);
                to_write.push(water_occupancy);
            }
        }
        to_write[0] = flag;

        if water_occupancy != 0x00 && count > 1 {
            /* Shorelines uses a new water occupancy value in the voxel data. Because of this,
            Roblox uses a hack to avoid having to reduce their 6 bits of material ID freedom
            by writing voxels with a count bit set to 1 and no count. This means we have to write
            all voxels in the run length manually. */
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
        let mut run_length_cursor = 0u16;
        let mut run_length_voxel = &base_voxel;
        for y in 0..CHUNK_SIZE {
            pos_cursor.0.y = y;
            for z in 0..CHUNK_SIZE {
                pos_cursor.0.z = z;
                for x in 0..CHUNK_SIZE {
                    pos_cursor.0.x = x;

                    let grabbed_voxel = match self.grid.get(&pos_cursor) {
                        Some(v) => v,
                        _ => &base_voxel,
                    };

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
            let cursor = match chunk_cursor {
                None => position,
                Some(c) => c,
            };
            let axes = [
                position.0.x - cursor.0.x,
                position.0.y - cursor.0.y,
                position.0.z - cursor.0.z,
            ];

            let mut negative_padding = 3;
            let mut negative_axes = [0x00, 0x00, 0x00];
            let mut adjusted_axes = [[0x00, 0x00, 0x00], [0x00, 0x00, 0x00], [0x00, 0x00, 0x00]];
            for (key, axis) in axes.iter().enumerate() {
                if *axis < 0 {
                    negative_axes[key] = 0xFF;
                }

                let axis_filler = match axis.abs() {
                    ..256 => 3,
                    256..65536 => 2,
                    65536.. => 1,
                };
                if axis_filler < negative_padding {
                    negative_padding = axis_filler;
                }

                // FIXME: This is really ugly
                let mut axis_adjuster = axis.abs();
                while axis_adjuster > 0 {
                    match axis_adjuster {
                        ..256 => {
                            adjusted_axes[2][key] = axis_adjuster as u8;
                            axis_adjuster -= axis_adjuster;
                        }
                        256..65536 => {
                            let offset = axis_adjuster / 256;
                            adjusted_axes[1][key] += offset as u8;
                            axis_adjuster -= offset * 256;
                        }
                        65536.. => {
                            let offset = axis_adjuster / 65536;
                            adjusted_axes[0][key] += offset as u8;
                            axis_adjuster -= offset * 65536;
                        }
                    }
                }
            }

            for _ in 0..negative_padding {
                data.extend(negative_axes.iter())
            }

            // 3 -> 1, 2 -> 2, 1 -> 3. Amount of 256 multiples to write
            for i in 0..(4 - negative_padding) {
                data.extend(adjusted_axes[2 - i].iter());
            }

            data.extend(chunk.encode());
            chunk_cursor = Some(position);
        }

        data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_default() {
        let mut terr = SmoothGrid::new();
        let mut chunk = Chunk::new_with_base(TerrainGridMaterial::Air);
        let mut voxel = Voxel::new_with_water(TerrainGridMaterial::Grass, 1.0, 0.5);
        for m in 2..=22 {
            voxel.set_material(TerrainGridMaterial::try_from(m as u8).unwrap());
            chunk.write_voxel(&VoxelCoordinates::new(m - 2, 0, 0), voxel);
        }
        terr.write_chunk(&ChunkCoordinates::default(), chunk.clone());
        terr.write_chunk(&ChunkCoordinates::new(1, 0, 0), chunk.clone());

        let encoded = base64::encode(terr.encode());
        println!("{}", encoded);
    }
}
