# Roblox Binary Model Format, Version 0
This document is based on:
- [*ROBLOX File Format* by Gregory Comer](http://www.classy-studios.com/Downloads/RobloxFileSpec.pdf)
- [LibRbxl by Gregory Comer](https://github.com/GregoryComer/LibRbxl)
- [rbxfile by Anaminus](https://github.com/RobloxAPI/rbxfile)
- [Roblox-File-Format by CloneTrooper1019](https://github.com/CloneTrooper1019/Roblox-File-Format)
- Observing `rbxm` and `rbxl` output from Roblox Studio

## Contents
- [Document Conventions](#document-conventions)
- [File Structure](#file-structure)
- [File Header](#file-header)
- [Chunks](#chunks)
	- [`META` Chunk](#meta-chunk)
	- [`SSTR` Chunk](#sstr-chunk)
	- [`INST` Chunk](#inst-chunk)
	- [`PROP` Chunk](#prop-chunk)
	- [`PRNT` Chunk](#prnt-chunk)
	- [`END` Chunk](#end-chunk)
- [Data Types](#data-types)
	- [String](#string)
	- [Bool](#bool)
	- [Int32](#int32)
	- [Float32](#float32)
	- [Float64](#float64)
	- [UDim](#udim)
	- [UDim2](#udim2)
	- [Ray](#ray)
	- [Faces](#faces)
	- [Axis](#axis)
	- [BrickColor](#brickcolor)
	- [Color3](#color3)
	- [Vector2](#vector2)
	- [Vector3](#vector3)
	- [CFrame](#cframe)
	- [Enum](#enum)
	- [Referent](#referent)
	- [Vector3int16](#vector3int16)
	- [NumberSequence](#numbersequence)
	- [ColorSequence](#colorsequence)
	- [NumberRange](#numberrange)
	- [Rect2D](#rect2d)
	- [PhysicalProperties](#physicalproperties)
	- [Color3uint8](#color3uint8)
	- [Int64](#int64)
	- [SharedString](#sharedstring)
- [Data Storage Notes](#data-storage-notes)
	- [Integer Transformations](#integer-transformations)
	- [Interleaved Array](#interleaved-array)
	- [Roblox Float Format](#roblox-float-format)

## Document Conventions
This document assumes a basic understanding of Rust's conventions for numeric types. For example:

- `u16` is an unsigned 16-bit integer
- `i32` is a signed 32-bit integer

Integers are assumed to be little endian and 2's complement unless otherwise specified. Big endian integers and integers with interesting transformations are present in this document and are explicitly noted.

## File Structure
1. File Header
2. Chunks
	1. Zero or one `META` chunks
	2. Zero or one `SSTR` chunks
	3. Zero or more `INST` chunk
	4. Zero or more `PROP` chunks
	5. One `PRNT` chunk
	6. One `END` chunk

## File Header
Every file starts with a 32 byte header.

| Field Name          | Format  | Value                                         |
|:--------------------|:--------|:----------------------------------------------|
| Magic Number        | 8 bytes | Always `<roblox!`                             |
| Signature           | 6 bytes | Always `89 ff 0a 1a 0a`                       |
| Version             | u16     | Always `0`                                    |
| Instance Type Count | i32     | Number of distinct instance types in the file |
| Instance Count      | i32     | Number of instances in the file               |
| Reserved            | 8 bytes | Always `0`                                    |

## Chunks
Every chunk starts with a 16 byte header followed by the chunk's data.

| Field Name          | Format  | Value                                             |
|:--------------------|:--------|:--------------------------------------------------|
| Chunk Name          | 4 bytes | The chunk's name, like `META` or `INST`           |
| Compressed Length   | u32     | Length of the chunk in bytes, if it is compressed |
| Uncompressed Length | u32     | Length of the chunk's data after decompression    |
| Reserved            | 4 bytes | Always `0`                                        |

If **chunk name** is less than four bytes, the remainder is filled with zeros.

If **compressed length** is zero, **chunk data** contains **uncompressed length** bytes of data for the chunk.

If **compressed length** is nonzero, **chunk data** contains an LZ4 compressed block. It is **compressed length** bytes long and will expand to **uncompressed length** bytes when decompressed.

When the **chunk data** is compressed, it is done so using the [LZ4](https://github.com/lz4/lz4) compression algorithm.

When documentation for individual chunks uses the term "chunk data", it refers to **chunk data** after it has been decompressed, if necessary.

### `META` Chunk
The `META` chunk has this layout:

| Field Name                 | Format         | Value                                       |
|:---------------------------|:---------------|:--------------------------------------------|
| Number of Metadata Entries | u32            | The number of metadata entries in the chunk |
| Metadata Entries           | Array(Entries) | The actual metadata entries                 |

Each metadata entry has the following format:

| Field Name     | Format | Value                                    |
|:---------------|:-------|:-----------------------------------------|
| Metadata Key   | String | The metadata key, which should be unique |
| Metadata Value | String | The value for this metadata key          |

The Metadata chunk (`META`) is a map of strings to strings. It represents metadata about the model, such as whether it was authored with `ExplicitAutoJoints` enabled.

There should be zero or one `META` chunks.

Observed metadata entries and their values:

- `ExplicitAutoJoints`: `true` or `false`

### `SSTR` Chunk
The `SSTR` chunk has this layout:

| Field Name          | Format                | Value                                        |
|:--------------------|:----------------------|:---------------------------------------------|
| Version             | u32                   | The version of the `SSTR` chunk (always `0`) |
| Shared String Count | u32                   | The number of SharedStrings in the chunk     |
| Strings             | Array(Shared Strings) | The actual shared string entries             |

A shared string entry looks like this:

| Field Name    | Format   | Value                                                                   |
|:--------------|:---------|:------------------------------------------------------------------------|
| MD5 Hash      | 16 bytes | An [MD5](https://en.wikipedia.org/wiki/MD5) hash of the `Shared String` |
| Shared String | String   | The string that's used by a later `PROP` chunk                          |

The Shared String chunk (`SSTR`) is an array of strings. It's used to reduce the overall size of a file by allowing large strings to be reused in [`PROP`](#prop-chunk) chunks. The `MD5 Hash` isn't used by Roblox Studio when loading the file.

There should be zero or one `SSTR` chunks.

### `INST` Chunk
The `INST` chunk has this layout:

| Field Name         | Format           | Value                                           |
|:-------------------|:-----------------|:------------------------------------------------|
| Type ID            | u32              | An arbitrarily-chosen ID to refer to this type  |
| Type Name          | String           | The type's name, like `Folder` or `Part`        |
| Object Format      | u8               | Always `0` (regular instance) or `1` (service)  |
| Number Instances   | u32              | The number of instances of this type            |
| Instance Referents | Array(Referent)  | The instance referents in the file of this type |
| Service Markers    | Array(u8)        | If **Object Format** is `1`, contains the byte `1` once for each instance in the chunk. Otherwise, not present. |

The Instance chunk (`INST`) defines a type of instance, how many of them there are in this file, and what referent IDs they have.

There should be one `INST` chunk for each type of instance defined.

There are two forms of the `INST` chunk determined by the **Object Format** field:

- `0`: regular
- `1`: service

If the **Object Format** is **regular**, the service markers section will not be present.

If the **Object Format** is **service**, the service markers section contains `1` repeated for the number of instances of that type in the file. If this field is not set, Roblox may create duplicate copies of services, like in [rojo-rbx/rbx-dom#11](https://github.com/rojo-rbx/rbx-dom/issues/11).

**Type ID** must be unique and ideally sorted monotonically among all `INST` chunks. It's used later in the file to refer to this type.

**Type Name** should match the `ClassName` specified on an instance in Roblox.

The length of the **Instance Referents** array must match the **Number of Instances** field.

### `PROP` Chunk
The `PROP` chunk has this layout:

| Field Name    | Format       | Value                                                      |
|:--------------|:-------------|:-----------------------------------------------------------|
| Type ID       | u32          | The type ID assigned in the `INST` chunk                   |
| Property Name | String       | The name of the property, like `CFrame`                    |
| Data Type     | u8           | The [Data Type](#data-types) of this property              |
| Values        | Array(value) | A list of values whose type is determined by **Data Type** |

The property chunk (`PROP`) defines a single property for a single instance type.

There should be one `PROP` chunk per property per instance type.

Because of the shape of this chunk, every instance of a given type must have the same properties specified with the same times. Put another way, if any instance in the file defines a property, all other instances of the same type must also specify that property!

**Type ID** defines the instance that this property applies to as defined in a preceding `INST` chunk.

**Property Name** defines the serializable name of the property. Note that this is not necessarily the same as the name reflected to Lua, which is sometimes referred to as the _canonical name_.

**Data Type** corresponds to a value from [Data Types](#data-types).

**Values** contains an array of values of **Data Type** whose length is the same as the number of instances with the type ID **Type ID**.

### `PRNT` Chunk
The `PRNT` chunk has this layout:

| Field Name        | Format          | Value                                     |
|:------------------|:----------------|:------------------------------------------|
| Version           | u8              | Always `0`                                |
| Number of Objects | u32             | Number of objects described in this chunk |
| Objects           | Array(Referent) | Objects to be parented                    |
| Parents           | Array(Referent) | Parents for objects                       |

The parent chunk (`PRNT`) defines the hierarchy relationship between every instance in the file.

There should be exactly one `PRNT` chunk.

**Version** field should currently always be zero.

**Number of Objects** should be the same as the number of instances in the file header chunk, since each object should have a parent.

**Object Array** and **Parent Array** should both have length equal to **Number of Objects**. The parent of the ID at position *N* in the **Object Array** is a child of the ID at position *N* in the **Parent Array**.

A null parent referent (`-1`) indicates that the object is a root instance. In a place, that means the object is a child of `DataModel`. In a model, that means the object should be placed directly under the object the model is being inserted into.

### `END` Chunk
The `END` chunk has this layout:

| Field Name  | Format  | Value              |
|:------------|:--------|:-------------------|
| Magic Value | 9 bytes | Always `</roblox>` |

The ending chunk (`END`) signifies the end of the file.

The `END` chunk must not be compressed. It is used as a rough form of file validation when uploading places to the Roblox website.

## Data Types

### String
**Type ID `0x01`**

| String |
| ------ |
| String length in bytes (u32) |
| Data |

String data is UTF-8 encoded.

### Bool
**Type ID `0x02`**

The `Bool` type is stored as a single byte. If the byte is `0x00`, the bool is `false`. If it is `0x01`, it is `true`.

When an array of Bools is present, they are stored in sequence.

### Int32
**Type ID `0x03`**

The `Int32` type is stored as a big-endian [transformed 32-bit integer](#integer-transformations).

When an array of Int32s is present, the bytes of the integers are subject to [byte interleaving](#interleaved-array).

### Float32
**Type ID 0x04**

The `Float32` type is stored using the [Roblox float format](#roblox-float-format) and is little-endian. This datatype is also called `float` or `single`.

When an array of Float32s is present, the bytes of the floats are subject to [byte interleaving](#interleaved-array).

### Float64
**Type ID 0x05**

The `Float64` type is stored using the [IEEE-754 format](https://en.wikipedia.org/wiki/Double-precision_floating-point_format) and is little-endian. This datatype is also called `double`.

When an array of Float64s is present, they are in sequence with no transformations.

### UDim
**Type ID 0x06**

### UDim2
**Type ID 0x07**

### Ray
**Type ID 0x08**

### Faces
**Type ID 0x09**

### Axis
**Type ID 0x0A**

### BrickColor
**Type ID 0x0B**

### Color3
**Type ID 0x0C**

### Vector2
**Type ID 0x0D**

### Vector3
**Type ID 0x0E**

### CFrame
**Type ID 0x10**

### Enum
**Type ID 0x12**

### Referent
**Type ID 0x13**

Referents are stored as transformed 32-bit signed integers. A value of `-1` (untransformed) indicates a null referent.

When reading an [Interleaved Array](#interleaved-array) of referents, they should be read accumulatively. In other words, the value of each referent id should be itself, plus its previous value.

Without accumulation, referents read from a file may look like this. This is **incorrect**:

- 1619
- 1
- 4
- 2
- 3
- 5

The **correct** interpretation of this data, with accumulation, is:

- 1619
- 1620
- 1624
- 1626
- 1629
- 1634

### Vector3int16
**Type ID 0x14**

### NumberSequence
**Type ID 0x15**

### ColorSequence
**Type ID 0x16**

### NumberRange
**Type ID 0x17**

### Rect2D
**Type ID 0x18**

### PhysicalProperties
**Type ID 0x19**

### Color3uint8
**Type ID 0x1A**

### Int64
**Type ID 0x1B**

The `Int64` type is stored as a big-endian [transformed 64-bit integer](#integer-transformations).

When an array of Int64s is present, the bytes of the integers are subject to [byte interleaving](#interleaved-array).

### SharedString
**Type ID 0x1C**

SharedStrings are stored as an [Interleaved Array](#interleaved-array) of [Int32s](#int32) that represent indices in the [`SSTR`](#sstr-chunk) string array.

Any property that's a [String](#string) can also be a SharedString.

## Data Storage Notes

### Integer Transformations

Some integers may be subject to a transformation to make them more compressable.

To transform an integer: if `x` greater than or equal to zero, transform it with `2 * x`. Otherwise, use `2 * |x| - 1`. In most compilers this is equivalent to `(x << 1) ^ (x >> 31)` for 32-bit integers. For 64-bit integers, the same format is used but with `63` instead of `31`. 

To untransform one: if `x` is divisble by 2, untransform it with `x / 2`. Otherwise, use `-(x + 1) / 2`. This is equivalent to `(x >> 1) ^ -(x & 1)`.

Untransforming with bitwise operators requires casting to an unsigned integer in some cases because `x >> 1` will result in a negative number if `x` is negative.

### Interleaved Array
Arrays of many types in property data have their bytes interleaved.

For example, an array of 4 bit integers normally represented as:

|||||||||||||
|--|--|--|--|--|--|--|--|--|--|--|--|
|**A0**|**A1**|**A2**|**A3**|B0|B1|B2|B3|C0|C1|C2|C3|

Would become, after interleaving:

|||||||||||||
|--|--|--|--|--|--|--|--|--|--|--|--|
|**A0**|B0|C0|**A1**|B1|C1|**A2**|B2|C2|**A3**|B3|C3|

Note that arrays of integers are generally subject to both interleaving and integer transformation.

### Roblox Float Format

Some data types do not follow the [IEEE-754 standard](https://en.wikipedia.org/wiki/Single-precision_floating-point_format) format for 32-bit floating point numbers. Instead, they use a proprietary format where the sign bit is after the mantissa.

| Format   | Bit Layout                            |
|:---------|:--------------------------------------|
| Standard | `seeeeeee emmmmmmm mmmmmmmm mmmmmmmm` |
| Roblox   | `eeeeeeee mmmmmmmm mmmmmmmm mmmmmmms` |

Where `s` is the sign bit, `e` is an exponent bit, and `m` is a mantissa bit.

As a practical example, below is a comparison of how `-0.15625` is stored:

| Format   | Binary View                           | Byte View     |
|:---------|:--------------------------------------|:--------------|
| Standard | `10111110 00100000 00000000 00000000` | `be 20 00 00` |
| Roblox   | `01111100 01000000 00000000 00000001` | `7c 40 00 01` |