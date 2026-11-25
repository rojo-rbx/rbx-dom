# BinaryString blobs

Certain properties (such as `Instance.Tags`) are serialized as blobs of binary data. These blobs are represented by Roblox as `BinaryString` values. The format used for a given property is entirely custom and must be known to read and write values to that property.

This document serves as unofficial documentation for potential `BinaryString` values, describing their structure.

## Encoding

Values of this type are encoded using the [`BinaryString`](xml.md#binarystring) data type in the [XML](xml.md) format. In the [binary](binary.md) file format, they are encoded as [`String`](binary.md#string) values.

## Blobs

The following is a list of `BinaryString` blobs and their formatting. For clarity, the name of the property using a blob is used as the header name, and the class it is a part of is listed beneath the header.

When a format is sufficiently complex, it may be stored in its own document for clarity.

### label
**Used By:** `AnimationRigData.label`

> **Note:** Reading this property requires `RobloxScriptSecurity`, meaning it is not accessible from ordinary scripts.

This blob serializes the set of rig labels for an `AnimationRigData` instance, as returned by `GetLabels()`. Each label is stored as a 32-bit identifier rather than a string directly — likely an index or hash into a separate label table.

If there are no labels, the blob is written as a fixed 8-byte header with a count of `0`: `01 00 00 00 00 00 00 00`.

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Version (?)   | Constant (`0x00000001`, `u32`).                        |
| `4`          | Label Count   | Total number of labels (`u32`).                        |

Followed by one entry per label:

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Label ID      | A 32-bit label identifier (`u32`).                     |

### name
**Used By:** `AnimationRigData.name`

> **Note:** Reading this property requires `RobloxScriptSecurity`, meaning it is not accessible from ordinary scripts.

This blob serializes the set of rig part/joint names for an `AnimationRigData` instance, as returned by `GetNames()`. Unlike `label`, names are stored as raw strings rather than IDs, using a separate-arrays layout: all name lengths are written first, followed by all name contents.

If there are no names, the blob is written as a fixed 8-byte header with a count of `0`: `01 00 00 00 00 00 00 00`.

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Version (?)   | Constant (`0x00000001`, `u32`).                        |
| `4`          | Name Count    | Total number of names (`u32`).                         |

Followed by one length entry per name:

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Name Length   | Length of the corresponding name in bytes (`u32`).     |

Followed immediately by the name contents themselves, in the same order as the lengths, with no padding or separators between them:

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `N`          | Name          | The name's raw bytes (`string`, not null-terminated).  |

### parent
**Used By:** `AnimationRigData.parent`

> **Note:** Reading this property requires `RobloxScriptSecurity`, meaning it is not accessible from ordinary scripts.

This blob serializes the parent-index table for an `AnimationRigData` instance, as returned by `GetParents()`. Each entry is a 16-bit index, likely referring to another entry's position within this same rig data (e.g. indicating which part/joint is the parent of the part/joint at a given index).

If there are no parents, the blob is written as a fixed 8-byte header with a count of `0`: `01 00 00 00 00 00 00 00`.

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Version (?)   | Constant (`0x00000001`, `u32`).                        |
| `4`          | Parent Count  | Total number of entries (`u32`).                        |

Followed by one entry per parent:

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `2`          | Parent Index  | A 16-bit index (`u16`).                                |

### postTransform / preTransform / transform
**Used By:** `AnimationRigData.postTransform`, `AnimationRigData.preTransform`, `AnimationRigData.transform`

> **Note:** Reading these properties requires `RobloxScriptSecurity`, meaning they are not accessible from ordinary scripts.

These blobs serialize a list of `CFrame` transforms — the pre-transform, post-transform, or base transform of each part/joint in an `AnimationRigData` instance, as returned by `GetPreTransforms()`, `GetPostTransforms()`, or `GetTransforms()` respectively. All three share the same underlying format.

If there are no transforms, the blob is written as a fixed 8-byte header with a count of `0`: `01 00 00 00 00 00 00 00`.

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Version (?)   | Constant (`0x00000001`, `u32`).                        |
| `4`          | Transform Count | Total number of transforms (`u32`).                  |

Followed by one 48-byte entry per transform, consisting of the CFrame's rotation matrix (as three basis vectors) and position, each written as three consecutive 32-bit floats:

| Size (Bytes) | Field         | Description                                       |
|:------------:|:--------------|:------------------------------------------------------|
| `12`         | X Basis       | The CFrame's X (right) basis vector: `R00, R01, R02` (`f32` × 3). |
| `12`         | Y Basis       | The CFrame's Y (up) basis vector: `R10, R11, R12` (`f32` × 3).    |
| `12`         | Z Basis       | The CFrame's Z (back) basis vector: `R20, R21, R22` (`f32` × 3).  |
| `12`         | Position      | The CFrame's position: `X, Y, Z` (`f32` × 3).                     |

### AttributeSerialized
**Used By:** `Instance.AttributesSerialize`

This blob is used to serialize [attributes][Attributes]. Due to the complexity of the format, a specification is located [here](attributes.md).

[Attributes]: https://create.roblox.com/docs/studio/instance-attributes

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
