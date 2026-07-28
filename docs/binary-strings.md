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

### ValuesAndTimes (FloatCurve)
**Used By:** `FloatCurve.ValuesAndTimes`

> **Note:** `HapticEffect.WaveformData` uses an identical format but has no `GetKeys` method (only `Set`), so it cannot be read back through this same path.

This blob serializes the keyframes of a `FloatCurve`, as returned by `GetKeys()`. It is composed of two sections: a **Values** section describing each key's value and tangents, followed by a shared **Times** section (see below) describing each key's timestamp.

If there are no keys, the blob is written as two fixed 8-byte headers, one for each section, each with a count of `0`:
`02 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00`

**Values section:**

| Size (Bytes) | Field       | Description                                            |
|:------------:|:------------|:------------------------------------------------------|
| `4`          | Version (?) | Constant (`0x00000002`, `u32`).                          |
| `4`          | Key Count   | Total number of keys (`u32`).                            |

Followed by one 14-byte entry per key:

| Size (Bytes) | Field           | Description                                            |
|:------------:|:----------------|:------------------------------------------------------|
| `1`          | Interpolation   | The key's `KeyInterpolationMode` enum value (`u8`).      |
| `1`          | Tangent Mode    | A bitmask describing which tangents were explicitly set on the key (see [Tangent Mode](#tangent-mode) below). |
| `4`          | Value           | The key's value (`f32`).                                 |
| `4`          | Left Tangent    | The key's left tangent (`f32`); derived if not explicitly set. |
| `4`          | Right Tangent   | The key's right tangent (`f32`); derived or mirrored if not explicitly set. |

This is followed immediately by the [Times section](#times-section).

### ValuesAndTimes (RotationCurve)
**Used By:** `RotationCurve.ValuesAndTimes`

This blob serializes the keyframes of a `RotationCurve`, as returned by `GetKeys()`, where each key's value is a `CFrame` converted to a quaternion. Like `FloatCurve`, it is composed of a Values section followed by a shared Times section.

If there are no keys, the blob is written as two fixed 8-byte headers, one for each section, each with a count of `0`:
`01 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00`

**Values section:**

| Size (Bytes) | Field       | Description                                            |
|:------------:|:------------|:------------------------------------------------------|
| `4`          | Version (?) | Constant (`0x00000001`, `u32`).                          |
| `4`          | Key Count   | Total number of keys (`u32`).                            |

Followed by one 25-byte entry per key:

| Size (Bytes) | Field           | Description                                            |
|:------------:|:----------------|:------------------------------------------------------|
| `1`          | Interpolation   | The key's `KeyInterpolationMode` enum value, offset by `12` (`u8`; i.e. `12 + Interpolation.Value`). The reason for this offset is unknown — it may distinguish rotation-curve interpolation modes from those of other curve types sharing the same byte space. |
| `4`          | Quaternion X    | The X component of the key's `CFrame`, converted to a quaternion (`f32`). |
| `4`          | Quaternion Y    | The Y component of the quaternion (`f32`).               |
| `4`          | Quaternion Z    | The Z component of the quaternion (`f32`).               |
| `4`          | Quaternion W    | The W component of the quaternion (`f32`).               |
| `4`          | Left Tangent    | The key's left tangent (`f32`), or `0` if not set.       |
| `4`          | Right Tangent   | The key's right tangent (`f32`), or `0` if not set.      |

Note that unlike `FloatCurve` and `ValueCurve`, `RotationCurve` does not derive missing tangents from neighboring keys — a missing tangent is simply written as `0`.

This is followed immediately by the [Times section](#times-section).

### ValuesAndTimes (ValueCurve)
**Used By:** `ValueCurve.ValuesAndTimes`

This blob serializes the keyframes of a `ValueCurve`, as returned by `GetKeys()`. Unlike `FloatCurve` and `RotationCurve`, a `ValueCurve` can hold values of varying [attribute-style types](attributes.md), so each key's value is tagged with a type ID and encoded using that type's corresponding encoder.

If there are no keys, **or** if the curve's `ValueType` cannot be resolved to a known, encodable type, the blob is written as two fixed 8-byte headers, one for each section, each with a count of `0`:
`02 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00`

**Values section:**

| Size (Bytes) | Field       | Description                                            |
|:------------:|:------------|:------------------------------------------------------|
| `4`          | Version (?) | Constant (`0x00000002`, `u32`).                          |
| `4`          | Key Count   | Total number of keys (`u32`).                            |

Followed by one variable-length entry per key:

| Size (Bytes) | Field           | Description                                            |
|:------------:|:----------------|:------------------------------------------------------|
| `1`          | Interpolation   | The key's `KeyInterpolationMode` enum value (`u8`).      |
| `1`          | Tangent Mode    | A bitmask describing which tangents were explicitly set on the key (see [Tangent Mode](#tangent-mode) below). |
| `4`          | Data Size       | The size, in bytes, of the Type ID field plus the encoded value that follows (`u32`; i.e. `1 + valueSize`). |
| `1`          | Type ID         | The [attribute type ID](attributes.md) corresponding to the curve's value type. |
| `N`          | Value           | The key's value, encoded using the type-specific encoder for the resolved value type ([see Attributes](attributes.md)). |
| `4`          | Left Tangent    | The key's left tangent (`f32`); derived or mirrored if not explicitly set. |
| `4`          | Right Tangent   | The key's right tangent (`f32`); derived or mirrored if not explicitly set. |

This is followed immediately by the [Times section](#times-section).

### ValuesAndTimes (MarkerCurve)
**Used By:** `MarkerCurve.ValuesAndTimes`

This blob serializes the markers of a `MarkerCurve`, as returned by `GetMarkers()`. Each marker's value is a null-terminated string rather than a numeric value.

If there are no markers, the blob is written as two fixed 8-byte headers, one for each section, each with a count of `0`:
`02 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00`

**Values section:**

| Size (Bytes) | Field         | Description                                            |
|:------------:|:--------------|:------------------------------------------------------|
| `4`          | Version (?)   | Constant (`0x00000002`, `u32`).                          |
| `4`          | Marker Count  | Total number of markers (`u32`).                          |

Followed by one entry per marker:

| Size (Bytes) | Field       | Description                                            |
|:------------:|:------------|:------------------------------------------------------|
| `N + 1`      | Value       | The marker's string value, followed by a single null byte (`0x00`) terminator. |

This is followed immediately by the [Times section](#times-section).

---

### Times section
Shared by `FloatCurve`, `RotationCurve`, `ValueCurve`, and `MarkerCurve` above, this section immediately follows each curve type's Values section and encodes the timestamp of each key/marker.

| Size (Bytes) | Field       | Description                                            |
|:------------:|:------------|:------------------------------------------------------|
| `4`          | Version (?) | Constant (`0x00000001`, `u32`).                          |
| `4`          | Key Count   | Total number of keys/markers (`u32`); matches the count in the preceding Values section. |

Followed by one entry per key/marker:

| Size (Bytes) | Field       | Description                                            |
|:------------:|:------------|:------------------------------------------------------|
| `4`          | Time        | The key's time, encoded as "ticks" (`i32`; see below).   |

**Time encoding:** a key's `Time` (in seconds, as a float) is converted to ticks by multiplying by `2400` and rounding to the nearest integer. If the scaled result falls outside the range of a signed 32-bit integer, it is clamped to `i32`'s minimum value (`-2147483648`) rather than wrapping or erroring.

---

### Tangent Mode
Used by `FloatCurve` and `ValueCurve` above (not `RotationCurve`, which always writes both tangents literally). This single byte is a bitmask indicating which of a key's `LeftTangent`/`RightTangent` were explicitly authored, versus needing to be derived or mirrored:

| Value | Left Tangent Set? | Right Tangent Set? | Resulting Behavior                                   |
|:-----:|:------------------:|:--------------------:|:--------------------------------------------------------|
| `0`   | No                  | No                    | Both tangents are derived from neighboring keys' interpolation and slope. |
| `1`   | Yes                 | No                    | The right tangent is mirrored from the left tangent.    |
| `2`   | No                  | Yes                   | The left tangent is mirrored from the right tangent.     |
| `3`   | Yes                 | Yes                   | Both tangents are used as explicitly authored, unchanged. |

The exact derivation rules for mode `0` (and the partial derivation feeding modes `1`/`2` before mirroring) depend on the key's own `Interpolation` mode and its neighbors', and are implementation details of the encoder rather than part of the wire format itself — see the "tangent derivation" logic used by the encoders for `FloatCurve` and `ValueCurve` respectively, which differ slightly from one another (e.g. in how `Constant` and `Linear` neighbors are treated).