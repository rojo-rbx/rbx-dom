# BinaryString blobs

Certain properties (such as `Instance.Tags`) are serialized as blobs of binary data. These blobs are represented by Roblox as `BinaryString` values. The format used for a given property is entirely custom and must be known to read and write values to that property.

This document serves as unofficial documentation for potential `BinaryString` values, describing their structure.

## Encoding

Values of this type are encoded using the [`BinaryString`](xml.md#binarystring) data type in the [XML](xml.md) format. In the [binary](binary.md) file format, they are encoded as [`String`](binary.md#string) values.

## Blobs

The following is a list of `BinaryString` blobs and their formatting. For clarity, the name of the property using a blob is used as the header name, and the class it is a part of is listed beneath the header.

When a format is sufficiently complex, it may be stored in its own document for clarity.

### AttributeSerialized
**Used By:** `Instance.AttributesSerialize`

This blob is used to serialize [attributes][Attributes]. Due to the complexity of the format, a specification is located [here](attributes.md).

[Attributes]: https://create.roblox.com/docs/studio/instance-attributes

### CollisionGroupData
**Used By:** `WorldRoot.CollisionGroupData`

This blob serializes the collision groups registered via [`PhysicsService`][PhysicsService]. It encodes, for each group, the group's ID, mask, and name.

If there are no registered collision groups, the blob is written as just the version and a count of `0`: `01 00`.

| Size (Bytes) | Field         | Description                                     |
|:------------:|:--------------|:-------------------------------------------------|
| `1`          | Version       | Constant (`0x01`).                                |
| `1`          | Group Count   | Total number of collision groups (`u8`).          |

Followed by one entry per group, in ascending ID order:

| Size (Bytes) | Field         | Description                                     |
|:------------:|:--------------|:-------------------------------------------------|
| `1`          | ID            | Group ID (`u8`), starting at `0`.                 |
| `1`          | Type ID       | Constant (`0x04`), denoting an `int32` value.     |
| `4`          | Mask          | Collision mask (`i32`, little-endian).            |
| `1`          | Name Length   | Length of the group name in bytes (`u8`).         |
| `N`          | Name          | Group name (`string`, not null-terminated).       |

As an example, given three collision groups:

1. Name: `"Default"`, ID: `0`, Mask: `-1`
2. Name: `"Group1"`, ID: `1`, Mask: `-3`
3. Name: `"Group2"`, ID: `2`, Mask: `-5`

The serialized data would be:
`01 03 00 04 FF FF FF FF 07 44 65 66 61 75 6C 74 01 04 FD FF FF FF 06 47 72 6F 75 70 31 02 04 FB FF FF FF 06 47 72 6F 75 70 32`

[PhysicsService]: https://create.roblox.com/docs/reference/engine/classes/PhysicsService#GetRegisteredCollisionGroups

### CollisionGroups
**Used By:** `Workspace.CollisionGroups`

> **Note:** This property is deprecated in favor of `WorldRoot.CollisionGroupData` (see above). It is documented here for compatibility with older files that may still use it.

This blob serializes the collision groups registered via [`PhysicsService`][PhysicsService] as a plain-text string, using `^` and `\` as delimiters — there is no binary length-prefixing.

Each collision group entry is formatted as:
`Name^Id^Mask`

Where `Name` is the group's name, `Id` is the group's ID (a zero-indexed integer based on the group's position in the registered list), and `Mask` is the group's collision mask, written as a plain decimal integer (which may be negative). Multiple entries are joined together using `\` as a separator; unlike the emote blobs above, there is no trailing delimiter after the final entry.

If there are no registered collision groups, the blob is an empty string.

As an example, given three collision groups:

1. Name: `"Default"`, ID: `0`, Mask: `-1`
2. Name: `"Group1"`, ID: `1`, Mask: `-3`
3. Name: `"Group2"`, ID: `2`, Mask: `-5`

The serialized data would be:
`Default^0^-1\Group1^1^-3\Group2^2^-5`

### MaterialColors
**Used By:** `Terrain.MaterialColors`

This blob is used to serialize [`MaterialColors`][MaterialColors].

`MaterialColors` is stored internally as `69` bytes, which is read a sequence of 23 three-byte arrays. Each of these arrays represents the red, green, and blue components of the color of a specific variant of the [`Material`][Material] enum.

These values are in a fixed order. The following table describes what byte (from the beginning of the blob) corresponds to what component of what `Material` enum value.

| `RR` | `GG` | `BB` | Material Variant  |
|:----:|:----:|:----:|:-----------------:|
| `00` | `01` | `02` | `None` (reserved) |
| `03` | `04` | `05` | `None` (reserved) |
| `06` | `07` | `08` | `Grass`           |
| `09` | `10` | `11` | `Slate`           |
| `12` | `13` | `14` | `Concrete`        |
| `15` | `16` | `17` | `Brick`           |
| `18` | `19` | `20` | `Sand`            |
| `21` | `22` | `23` | `WoodPlanks`      |
| `24` | `25` | `26` | `Rock`            |
| `27` | `28` | `29` | `Glacier`         |
| `30` | `31` | `32` | `Snow`            |
| `33` | `34` | `35` | `Sandstone`       |
| `36` | `37` | `38` | `Mud`             |
| `39` | `40` | `41` | `Basalt`          |
| `42` | `43` | `44` | `Ground`          |
| `45` | `46` | `47` | `CrackedLava`     |
| `48` | `49` | `50` | `Asphalt`         |
| `51` | `52` | `53` | `Cobblestone`     |
| `54` | `55` | `56` | `Ice`             |
| `57` | `58` | `59` | `LeafyGrass`      |
| `60` | `61` | `62` | `Salt`            |
| `63` | `64` | `65` | `Limestone`       |
| `66` | `67` | `68` | `Pavement`        |

The first two rows appear to be unused at this moment and should always be written as `00 00 00` to preserve compatibility.

[MaterialColors]: https://create.roblox.com/docs/reference/engine/classes/Terrain#MaterialColors
[Material]: https://create.roblox.com/docs/reference/engine/enums/Material

### Tags
**Used By:** `Instance.Tags`

This blob is used to serialize [`CollectionService`][CollectionService] tags for an `Instance`.

`Tags` is stored as an array of bytes representing every tag on an `Instance`. The array is delineated using `00`. Otherwise, the literal bytes of the tag are written.

As an example, an `Instance` that had the tags `Hello`, `from`, and `Rojo` would have them serialized as follows:

`48 65 6C 6C 6F 00 66 72 6F 6D 00 52 6F 6A 6F`

The `Tags` blob may be stored in the SharedString index. Instances with identical sets of Tags share the same SharedString entry.

[CollectionService]: https://create.roblox.com/docs/reference/engine/classes/CollectionService
