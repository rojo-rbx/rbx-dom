# Roblox Terrain Binary Format

This document describes the Terrain binary format. In this format there is no field for Version, so it is assumed that any future changes will be additions to the existing format or a new format entirely. This specification does not include the adjacent PhysicsGrid binary format, only SmoothGrid.

## Contents

- [Document Conventions](#document-conventions)
  - [Byte Interleaving](#byte-interleaving)
- [File Structure](#file-structure)
- [Data Types](#data-types)
  - [Chunk](#chunk)
  - [Voxel](#voxel)
  - [Voxel.Flag](#voxelflag)
  - [Material](#material)

## Document Conventions

This document assumes a basic understanding of Rust's convention for numeric types. For example:

- `u32` is an unsigned 32-bit integer
- `f32` is a 32-bit floating point number

All numeric types are little endian. Floats are stored as a dividend of their type's numeric maximum; as such, a `f8` with a value of `0x31` translates to `0x31 / 0xFF`, or `~0.192`. Floats are unsigned.

Unless otherwise noted, all structs in this document are assumed to be stored with their components in the sequence listed without any modification. That is, if a struct is listed as being composed of an `i32` and an `f32`, it can be assumed that it is stored as an `i32` followed by an `f32`.

### Byte Interleaving

When stored as arrays, some data types have their bytes interleaved to help with compression. Byte interleaving is also known as byte shuffling in SIMD. Cases where byte interleaving is present are explicitly noted.

When the bytes of an array are interleaved, they're stored with the first bytes all in sequence, then the second bytes, then the third, and so on. As an example, the sequence `A0 A1 B0 B1 C0 C1` is stored as `A0 B0 C0 A1 B1 C1`.

Viewed another way, it means that the bytes are effectively stored in 'columns' rather than 'rows'. If an array of four 32-bit integers were viewed as a 4x4 matrix, for example, it would normally look like this:

|       | Column 1 | Column 2 | Column 3 | Column 4 |
|:-----:|:--------:|:--------:|:--------:|:--------:|
| Row 1 | `A0`     | `A1`     | `A2`     | `A3`     |
| Row 2 | `B0`     | `B1`     | `B2`     | `B3`     |
| Row 3 | `C0`     | `C1`     | `C2`     | `C3`     |
| Row 4 | `D0`     | `D1`     | `D2`     | `D3`     |

When interleaved, the same array would instead look like this:

|       | Column 1 | Column 2 | Column 3 | Column 4 |
|:-----:|:--------:|:--------:|:--------:|:--------:|
| Row 1 | `A0`     | `B0`     | `C0`     | `D0`     |
| Row 2 | `A1`     | `B1`     | `C1`     | `D1`     |
| Row 3 | `A2`     | `B2`     | `C2`     | `D2`     |
| Row 4 | `A3`     | `B3`     | `C3`     | `D3`     |

## File Structure

The first two bytes of the blob are `0x01`, which is a version number, followed by `0x05`, which is a logarithm base 2 of the chunk size in voxels. The default chunk size is 32<sup>3</sup> (32768). The amount of voxels in a chunk can be described as <code>(2<sup>Chunk Size</sup>)<sup>3</sup></code>.

Size values other than `0x05` are never written by Studio, but values between `0x00` (inclusive) and `0x08` (inclusive) can still be deserialized by the engine, giving a theoretical range of possible chunk sizes between 1<sup>3</sup> and 256<sup>3</sup>. However, the engine splits chunks differing in size from 32<sup>3</sup> back into the default chunk size. Any size value other than `0x05` has poorly tested and undefined behavior.

Immediately following the header is an array of chunks, each of which must contain enough voxels to reach the maximum count, which is equivalent to the chunk size. Chunks are ascendingly ordered by X, then Y, then Z based on their position in the world. Each chunk represents a cube of 128<sup>3</sup> units in world space.

| Field Name     | Format                 | Value                                                                                |
| :------------- | :--------------------- | :----------------------------------------------------------------------------------- |
| Version Number | `u8`                   | The binary format version used. Currently, only `0x01` is supported by the engine.   |
| Chunk Size     | `u8`                   | Logarithm base 2 of the chunk size. Should be `0x05`.                                |
| Chunks         | [Vec\<Chunk\>](#chunk) | Dynamic number of chunks. Runs until the end of the blob. Unknown/no maximum amount. |

## Data Types

Terrain data is represented using a variety of different data types. Data described within them follows any conventions set in the [document conventions](#document-conventions).

### Chunk

The `Chunk` type is stored with a dynamic size dependent on the size of the voxels contained within, and the end of its data is marked by reaching its maximum voxel count dependent on chunk size. Voxels are ascendingly ordered by Y, then Z, then X based on their position in the chunk. Voxels are stored in rows of 32 units on each axis, each representing 4 units in world space.

Voxel data is preceded by the offset between this chunk and the last in the blob, or from `0, 0, 0` if this is the first chunk in the blob. This offset is stored using 3 `i32`s (in XYZ order) stored in an [interleaved](#byte-interleaving) format.

Chunks have a maximum world position in chunk space of 262,144 on each axis. After this point, voxels are located in a region (above 2<sup>23</sup> in world space) where floating point precision devolves to `1.0`, and the engine refuses to load a blob that contains chunks beyond this point.

| Field Name    | Format                                        | Value                                                                                  |
| :------------ | :-------------------------------------------- | :------------------------------------------------------------------------------------- |
| Offset        | [Interleaved<[i32; 3]>]((#byte-interleaving)) | Offset from the last chunk in the blob, each axis being in singular chunks.            |
| Voxels        | [Vec\<Voxel\>](#voxel)                        | Dynamic number of voxels, running until the chunk size is reached.                     |

### Voxel

The `Voxel` type is stored with a dynamic size (between 1-4 bytes) dependent on its set [bitflags](#voxelflag). Only stored within [chunks](#chunk). A reference for the data contained within voxels [can be found here][Terrain.WriteVoxelChannels]. Empty voxels are stored with their material set to Air.

Voxels are stored within a chunk using [run-length encoding]. A count of `1` would indicate the voxel repeating itself once, meaning 2 of the same voxel in a row would be read when decoded. Occupancy values are stored using the float format described in the [document conventions](#document-conventions).

Water occupancy is dynamically set based on the `Store Count` flag and the stored `Count` value. If `Store Count` is enabled and `Count` is `0x00`, then the `Water Occupancy` is written.

If the solid occupancy of a voxel is `1.0` and the set material is not `Air`, the water occupancy must always be `0.0` if it's set. If the water occupancy of a voxel is `1.0` and the solid occupancy is `0.0`, the voxel should be stored like a solid voxel with its material set to `Water`, with no `Water Occupancy` being written.

| Field Name                    | Format                    | Value                                                                                                                  |
| :---------------------------- | :------------------------ | :--------------------------------------------------------------------------------------------------------------------- |
| Flag                          | [Voxel.Flag](#voxelflag)  | Contains the material of this voxel, along with other bitflags.                                                         |
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

The `Material` enum is used for the `Material Index` value of a [Voxel.Flag](#voxelflag) to set the material of a [Voxel](#voxel). Constitutes a terrain-specific subset of Roblox's [Enum.Material].

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
