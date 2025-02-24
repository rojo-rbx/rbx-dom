# Roblox Terrain Binary Format

This document describes the Terrain binary format. In this format there is no field for Version, so it is assumed that any future changes will be additions to the existing format or a new format entirely. This specification does not include the adjacent PhysicsGrid binary format, only SmoothGrid.

# Contents

- [Document Conventions](#document-conventions)
- [File Structure](#file-structure)
- [Data Types](#data-types)
  - [Chunk](#chunk)
  - [Voxel](#voxel)
  - [Voxel.Flag](#voxel.flag)
  - [Material](#material)

## Document Conventions

This document assumes a basic understanding of Rust's convention for numeric types. For example:

- `u32` is an unsigned 32-bit integer
- `f32` is a 32-bit floating point number

All numeric types are little endian. Floats are stored as a dividend of their type's numeric maximum; as such, a `f8` with a value of `0x31` translates to `0x31 / 0xFF`, or `~0.192`. Floats are unsigned.

Unless otherwise noted, all structs in this document are assumed to be stored with their components in the sequence listed without any modification. That is, if a struct is listed as being composed of an `i32` and an `f32`, it can be assumed that it is stored as an `i32` followed by an `f32`.

## File Structure

The first two bytes of the blob are `0x01`, which is a magic number, followed by `0x05`, which is a logarithm base 2 of the chunk size in voxels. The default chunk size is 32<sup>3</sup> (32768). The amount of voxels in a chunk can be described as <code>(2<sup>Chunk Size</sup>)<sup>3</sup></code>.

Size values other than `0x05` are normally not written by Studio, but values between `0x00` (inclusive) and `0x08` (inclusive) can still be deserialized by the engine, giving a theoretical range of possible chunk sizes between 1<sup>3</sup> and 256<sup>3</sup>. However, the engine splits chunks differing in size from 32<sup>3</sup> back into the default chunk size. Any size value other than `0x05` has poorly tested and undefined behavior.

Immediately following the header is an array of chunks, each of which must contain enough voxels to reach the maximum count, which is equivalent to the chunk size. Chunks are ascendingly ordered by X, then Y, then Z based on their position in the world. Each chunk represents a cube of 128<sup>3</sup> units in world space.

| Field Name   | Format               | Value                                                                                |
| :----------- | :------------------- | :----------------------------------------------------------------------------------- |
| Magic Number | `u8`                 | The magic number `0x01`                                                              |
| Chunk Size   | `u8`                 | Logarithm base 2 of the chunk size. Should be `0x05`.                                |
| Chunks       | [Vec<Chunk>](#chunk) | Dynamic number of chunks. Runs until the end of the blob. Unknown/no maximum amount. |

## Data Types

Terrain data is represented using a variety of different data types. Data described within them follows any conventions set in the [document conventions](#document-conventions).

### Chunk

The `Chunk` type is stored with a dynamic size dependent on the size of the voxels contained within, and the end of its data is marked by reaching its maximum voxel count dependent on chunk size. Voxels are ascendingly ordered by Y, then Z, then X based on their position in the chunk. Voxels are stored in rows of 32 units on each axis, each representing 4 units in world space.

Voxel data is preceded by the offset between this chunk and the last in the blob, or from `0, 0, 0` if this is the first chunk in the blob. This offset is stored using 3 vectors of `(x: u8, y: u8, z: u8)`, with 0xFF values in _all_ unused offsets (and the `Signedness` value) indicating a negative sign. Using the sign information from the prior vectors, the range of an axis is `-0xFF` to `0xFF`, unlike conventional signed integers. All coordinates are in chunk space (increments of 1 chunk), not world space.

The actual offset is the sum of the three offset vectors, multiplied by their respective powers of 256. For example, given chunk B, with a real offset from chunk A on the X axis of 65,797 chunks, chunk B's offset would be stored as follows:

1. Signedness: `(0x00, 0x00, 0x00)`
2. Offset (65536): Divide and floor 65,797 by 65,536. `(0x01, 0x00, 0x00)`
3. Offset (256): Subtract 65,536 from 65,797. Divide and floor 261 by 256. `(0x01, 0x00, 0x00)`
4. Offset (1): Subtract 256 from 261. `(0x05, 0x00, 0x00)`

As an example for signedness, given two chunks, one at a position of `(2, 0, 0)` and another at a position of `(4, 0, -1)` in chunk space, the latter's offset would be stored as follows:

1. Signedness: `(0x00, 0x00, 0xFF)`
2. Offset (65536): `(0x00, 0x00, 0xFF)`
3. Offset (256): `(0x00, 0x00, 0xFF)`
4. Offset (1): `(0x02, 0x00, 0x01)`

Chunks have a maximum world position in chunk space of 262,144 on each axis. After this point, voxels are located in a region (above 2<sup>23</sup> in world space) where floating point precision devolves to `1.0`, and the engine refuses to load a blob that contains chunks beyond this point.

| Field Name     | Format               | Value                                                                               |
| :------------- | :------------------- | :---------------------------------------------------------------------------------- |
| Signedness     | `[u8; 3]`            | Always the signedness of the following vectors. 0xFF in negative axes.              |
| Offset (65536) | `[i8; 3]`            | Offset from the last chunk in the blob, each axis being multiplied by 65536 chunks. |
| Offset (256)   | `[i8; 3]`            | Offset from the last chunk in the blob, each axis being multiplied by 256 chunks.   |
| Offset (1)     | `[i8; 3]`            | Offset from the last chunk in the blob, each axis being in singular chunks.         |
| Voxels         | [Vec<Voxel>](#voxel) | Dynamic number of voxels, running until the chunk size is reached.                  |

### Voxel

The `Voxel` type is stored with a dynamic size (between 1-4 bytes) dependent on its set [bitflags](#voxel.flag). Only stored within [chunks](#chunk). A reference for the data contained within voxels [can be found here][Terrain.WriteVoxelChannels]. Empty voxels are stored with their material set to Air.

Voxels are stored within a chunk using [run-length encoding]. A count of `1` would indicate the voxel repeating itself once, meaning 2 of the same voxel in a row would be read when decoded. Occupancy values are stored using the float format described in the [document conventions](#document-conventions).

Water occupancy is dynamically set based on the `Store Count` flag and the stored `Count` value. If `Store Count` is enabled and `Count` is `0x00`, then the `Water Occupancy` is written.

If the solid occupancy of a voxel is `1.0` and the set material is not `Air`, the water occupancy must always be `0.0` if it's set. If the water occupancy of a voxel is `1.0` and the solid occupancy is `0.0`, the voxel should be stored like a solid voxel with its material set to `Water`, with no `Water Occupancy` being written.

| Field Name                    | Format                    | Value                                                                                                                  |
| :---------------------------- | :------------------------ | :--------------------------------------------------------------------------------------------------------------------- |
| Flag                          | [Voxel.Flag](#voxel.flag) | Contains the material of this voxel, along with other bitflags.                                                        |
| Solid Occupancy               | `f8`                      | Represents how full of a solid material the voxel is between 0-100%. Only stored if the `Store Occupancy` flag is set. |
| Count                         | `u8`                      | Run-length count. Only stored if the `Store Count` flag is set.                                                        |
| [Water Occupancy][Shorelines] | `f8`                      | Represents how full of Water the voxel is between 0-100%. Only stored based on the conditions described above.         |

[run-length encoding]: https://en.wikipedia.org/wiki/Run-length_encoding
[Terrain.WriteVoxelChannels]: https://create.roblox.com/docs/reference/engine/classes/Terrain#WriteVoxelChannels
[Shorelines]: https://devforum.roblox.com/t/shorelines-full-release/2952103

### Voxel.Flag

The `Voxel.Flag` subtype is a 1-byte (unsigned) bitflag describing the data written to a voxel. The occupancy bit is only set if the voxel's solid occupancy is not `1.0`. The count bit is only set if the voxel's water occupancy is not `0.0`, or the voxel has a run-length count of `1` or above.

The following description is in order from least to most significant bits.

| Flag Name       | Bits | Value                                                                                  |
| :-------------- | :--- | :------------------------------------------------------------------------------------- |
| Material Index  | 6    | Integer index of this voxel's material. See the [Material](#material) enum for values. |
| Store Occupancy | 1    | Boolean for whether we should store this voxel's solid occupancy as a byte.            |
| Store Count     | 1    | Boolean for whether we should store this voxel's run-length count as a byte.           |

### Material

The `Material` enum is used for the `Material Index` value of a [Voxel.Flag](#voxel.flag) to set the material of a [Voxel](#voxel). Constitutes a terrain-specific subset of Roblox's [Enum.Material].

| Material    | Value |
| :---------- | :---- |
| Air         | 0     |
| Water       | 1     |
| Grass       | 2     |
| Slate       | 3     |
| Concrete    | 4     |
| Brick       | 5     |
| Sand        | 6     |
| WoodPlanks  | 7     |
| Rock        | 8     |
| Glacier     | 9     |
| Snow        | 10    |
| Sandstone   | 11    |
| Mud         | 12    |
| Basalt      | 13    |
| Ground      | 14    |
| CrackedLava | 15    |
| Asphalt     | 16    |
| Cobblestone | 17    |
| Ice         | 18    |
| LeafyGrass  | 19    |
| Salt        | 20    |
| Limestone   | 21    |
| Pavement    | 22    |

[Enum.Material]: https://create.roblox.com/docs/reference/engine/enums/Material
