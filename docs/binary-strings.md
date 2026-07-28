# BinaryString blobs

Certain properties (such as `Instance.Tags`) are serialized as blobs of binary data. These blobs are represented by Roblox as `BinaryString` values. The format used for a given property is entirely custom and must be known to read and write values to that property.

This document serves as unofficial documentation for potential `BinaryString` values, describing their structure.

## Encoding

Values of this type are encoded using the [`BinaryString`](xml.md#binarystring) data type in the [XML](xml.md) format. In the [binary](binary.md) file format, they are encoded as [`String`](binary.md#string) values.

## Blobs

The following is a list of `BinaryString` blobs and their formatting. For clarity, the name of the property using a blob is used as the header name, and the class it is a part of is listed beneath the header.

When a format is sufficiently complex, it may be stored in its own document for clarity.

### AngleAttenuation / DistanceAttenuation
**Used By:** `AudioEmitter.AngleAttenuation`, `AudioEmitter.DistanceAttenuation`, `AudioListener.AngleAttenuation`, `AudioListener.DistanceAttenuation`

These blobs serialize an attenuation curve — a set of key-value pairs mapping an angle or distance (the key) to a volume level (the value), as returned by `GetAngleAttenuation()` / `GetDistanceAttenuation()`. All four properties share the same underlying format.

If the attenuation curve has no points, the blob is a single zero byte: `00`.

| Size (Bytes) | Field         | Description                                          |
|:------------:|:--------------|:-------------------------------------------------------|
| `1`          | Version (?)   | Constant (`0x00`).                                     |

Followed by one entry per curve point, sorted in ascending order by key:

| Size (Bytes) | Field         | Description                                          |
|:------------:|:--------------|:-------------------------------------------------------|
| `4`          | Key           | The angle or distance value (`f32`).                   |
| `4`          | Volume        | The volume at this key (`f32`).                        |

As an example, a distance attenuation curve with two points — `0` mapped to volume `1.0`, and `50` mapped to volume `0.0` — would be serialized as:

```
00 00 00 00 00 00 00 80 3F 00 00 48 42 00 00 00 00
```

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
