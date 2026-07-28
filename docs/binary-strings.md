# BinaryString blobs

Certain properties (such as `Instance.Tags`) are serialized as blobs of binary data. These blobs are represented by Roblox as `BinaryString` values. The format used for a given property is entirely custom and must be known to read and write values to that property.

This document serves as unofficial documentation for potential `BinaryString` values, describing their structure.

## Encoding

Values of this type are encoded using the [`BinaryString`](xml.md#binarystring) data type in the [XML](xml.md) format. In the [binary](binary.md) file format, they are encoded as [`String`](binary.md#string) values.

## Blobs

The following is a list of `BinaryString` blobs and their formatting. For clarity, the name of the property using a blob is used as the header name, and the class it is a part of is listed beneath the header.

When a format is sufficiently complex, it may be stored in its own document for clarity.

### AccessoryBlob
**Used By:** `HumanoidDescription.AccessoryBlob`

This blob serializes the accessories worn by a `Humanoid` (via `Humanoid:GetAccessories(false)`) as a JSON-encoded array. Unlike other `BinaryString` blobs in this document, this format is plain JSON text rather than a custom binary layout.

Each element of the array is an object with the following fields:

| Field            | Type      | Description                                                        |
|:-----------------|:----------|:---------------------------------------------------------------------|
| `AssetId`        | `number`  | The asset ID of the accessory.                                       |
| `Order`          | `number`  | The accessory's `Order` value, controlling layering/render order.    |
| `AccessoryType`  | `string`  | The name of the accessory's [`AccessoryType`][AccessoryType] enum value (e.g. `"Hair"`, `"Hat"`). |
| `Puffiness`      | `number`  | The accessory's `Puffiness` value.                                    |

If the `Humanoid` has no accessories, the blob is an empty JSON array: `[]`.

As an example, a `Humanoid` with a single hat accessory might serialize as:

```json
[{"AssetId":1234567,"Order":0,"AccessoryType":"Hat","Puffiness":0.5}]
```

[AccessoryType]: https://create.roblox.com/docs/reference/engine/enums/AccessoryType

### EmotesDataInternal
**Used By:** `HumanoidDescription.EmotesDataInternal`

This blob serializes the set of emotes known to a `HumanoidDescription` (via `HumanoidDescription:GetEmotes()`), mapping each emote name to one or more associated asset IDs. The format is a plain-text string using `^` and `\` as delimiters — there is no binary length-prefixing.

Each emote entry is formatted as:
`Name^Id1^Id2^...^IdN^\`

Where `Name` is the emote's name and each `Id` is an asset ID associated with that emote (an emote may have multiple IDs, e.g. for variants). Each entry, including the last, is terminated with a trailing `^\` — entries are not joined with a separator, they are simply concatenated one after another, each ending in its own `^\`.

If a `HumanoidDescription` has no emotes, the blob is an empty string.

As an example, a `HumanoidDescription` with two emotes, `"Wave"` (single ID) and `"Dance"` (two IDs), would be serialized as:
`Wave^123456^\Dance^234567^234568^\`

### EquippedEmotesDataInternal
**Used By:** `HumanoidDescription.EquippedEmotesDataInternal`

This blob serializes the emotes currently equipped on a `HumanoidDescription` (via `HumanoidDescription:GetEquippedEmotes()`), along with the hotbar slot each is equipped to. Like `EmotesDataInternal`, this is a plain-text format using `^` and `\` as delimiters.

Each equipped emote entry is formatted as:
`Slot^Name\`

Where `Slot` is the numeric hotbar slot the emote is equipped to, and `Name` is the emote's name. Each entry, including the last, is terminated with a trailing `\` — entries are not joined with a separator, they are simply concatenated one after another, each ending in its own `\`.

If no emotes are equipped, the blob is an empty string.

As an example, a `HumanoidDescription` with `"Wave"` equipped to slot `1` and `"Dance"` equipped to slot `2` would be serialized as:

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
