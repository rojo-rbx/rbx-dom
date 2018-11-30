# Roblox Binary Model Format, Version 0
This document is based on:
- [*ROBLOX File Format* by Gregory Comer](http://www.classy-studios.com/Downloads/RobloxFileSpec.pdf)
- [LibRbxl by Gregory Comer](https://github.com/GregoryComer/LibRbxl)
- [rbxfile by Anaminus](https://github.com/RobloxAPI/rbxfile)
- [Rbx2Source BinaryFormat by CloneTrooper1019](https://github.com/CloneTrooper1019/Rbx2Source/blob/bcc20595cacaa9fdb8bf993bd954abb8c9d0af34/src/Reflection/BinaryFormat/)
- Observing `rbxm` and `rblx` output from Roblox Studio

## Contents
- [File Structure](#file-structure)
- [File Header](#file-header)
- [Chunks](#chunks)
	- [`META` Chunk](#meta-chunk)
	- [`INST` Chunk](#inst-chunk)
	- [`PROP` Chunk](#prop-chunk)
	- [`PRNT` Chunk](#prnt-chunk)
	- [`END` Chunk](#end-chunk)
- [Data Types](#data-types)
	- [String](#string)
	- [Integer](#integer)
	- [Interleaved Array](#interleaved-array)
	- [Referent](#referent)

## File Structure
1. File Header
2. Chunks
	1. Zero or one `META` chunks
	2. Zero or more `INST` chunk
	3. Zero or more `PROP` chunks
	4. One `PRNT` chunk
	5. One `END` chunk

## File Header
Every file starts with:

<table>
	<tr>
		<th width="40">0</th>
		<th width="40">1</th>
		<th width="40">2</th>
		<th width="40">3</th>
		<th width="40">4</th>
		<th width="40">5</th>
		<th width="40">6</th>
		<th width="40">7</th>
		<th width="40">8</th>
		<th width="40">9</th>
		<th width="40">10</th>
		<th width="40">11</th>
		<th width="40">12</th>
		<th width="40">13</th>
		<th width="40">14</th>
		<th width="40">15</th>
	</tr>
	<tr>
		<td colspan="8">Magic number (<code>&lt;roblox!</code>)</td>
		<td colspan="6">Signature (<code>89 ff 0d 0a 1a 0a</code>)</td>
		<td colspan="2">Version (<code>0</code>)</td>
	</tr>
	<tr>
		<td colspan="4">Number of classes (<code>u32</code>)</td>
		<td colspan="4">Number of instances (<code>u32</code>)</td>
		<td colspan="8">Always zero</td>
	</tr>
</table>

## Chunks
Every chunk starts with:

<table>
	<tr>
		<th width="40">0</th>
		<th width="40">1</th>
		<th width="40">2</th>
		<th width="40">3</th>
		<th width="40">4</th>
		<th width="40">5</th>
		<th width="40">6</th>
		<th width="40">7</th>
		<th width="40">8</th>
		<th width="40">9</th>
		<th width="40">10</th>
		<th width="40">11</th>
		<th width="40">12</th>
		<th width="40">13</th>
		<th width="40">14</th>
		<th width="40">15</th>
	</tr>
	<tr>
		<td colspan="4">Chunk name</td>
		<td colspan="4">Compressed length (<code>u32</code>)</td>
		<td colspan="4">Uncompressed length (<code>u32</code>)</td>
		<td colspan="4">Always zero</td>
	</tr>
	<tr>
		<td colspan="16">Chunk data</td>
	</tr>
</table>

If **chunk name** is less than four bytes, the remainder is filled with zeros.

If **compressed length** is zero, **chunk data** contains **uncompressed length** bytes of data for the chunk.

If **compressed length** is nonzero, **chunk data** contains an LZ4 compressed block. It is **compressed length** bytes long and will expand to **uncompressed length** bytes when decompressed.

When documentation for individual chunks uses the term "chunk data", it refers to **chunk data** after it has been decompressed, if necessary.

### `META` Chunk
| `META` Chunk Data |
| ----------------- |
| Number of entries (`u32`) |
| Metadata Entries (fills rest of chunk) |

| Metadata Entry |
| ----- |
| Key ([String](#string)) |
| Value ([String](#string)) |

The Metadata chunk (`META`) is a map of strings to strings. It represents metadata about the model, such as whether it was authored with `ExplicitAutoJoints` enabled.

### `INST` Chunk
| `INST` Chunk Data |
| ----------------- |
| Type ID (`u32`) |
| Type Name ([String](#string)) |
| Additional data marker (`u8`) |
| Number instances (`u32`) |
| Instance referents ([Interleaved Array](#interleaved-array) of [Referent](#referent)) |

TODO: More detailed information

### `PROP` Chunk
| `PROP` Chunk Data |
| ----------------- |
| Type ID (`u32`) |
| Property name ([String](#string)) |
| Data type (`u8`) |
| Values (interleaved array of data) |

TODO: More detailed information

### `PRNT` Chunk
| `PRNT` Chunk Data |
| ----------------- |
| Number objects (`u32`) |
| Objects ([Interleaved Array](#interleaved-array) of [Referent](#referent)) |
| Parents ([Interleaved Array](#interleaved-array) of [Referent](#referent)) |

TODO: More detailed information

### `END` Chunk
| `END` Chunk Data |
| ---------------- |
| Magic value `</roblox>` |

The `END` chunk should not be compressed.

## Data Types

### String
| String |
| ------ |
| String length in bytes (u32) |
| Data |

String data is UTF-8 encoded.

### Integer
**Untransformed integers**, generally in header data, are little-endian and two's complement. Integers are untransformed unless denoted otherwise.

**Transformed integers**, normally used in property data, are big-endian and are transformed and untransformed via:

```rust
fn transform_i32(value: i32) -> i32 {
	if value >= 0 {
		value * 2
	} else {
		2 * -value - 1
	}
}

fn untransform_i32(value: i32) -> i32 {
	if value % 2 == 0 {
		value / 2
	} else {
		-(value +1 1) / 2
	}
}
```

Integers can also be transformed via bitwise ops to avoid branches:

```rust
fn transform_i32(value: i32) -> i32 {
	(value << 1) ^ (value >> 31)
}

fn untransform_i32(value: i32) -> i32 {
	((value as u32) >> 1) as i32 ^ -(value & 1)
}
```

### Interleaved Array
Arrays of many types in property data have their bytes interleaved.

For example, an array of 4 bit integers normally represented as:

<table>
	<tr>
		<td><b>A0</b></td>
		<td><b>A1</b></td>
		<td><b>A2</b></td>
		<td><b>A3</b></td>
		<td>B0</td>
		<td>B1</td>
		<td>B2</td>
		<td>B3</td>
		<td>C0</td>
		<td>C1</td>
		<td>C2</td>
		<td>C3</td>
	</tr>
</table>

Would become, after interleaving:

<table>
	<tr>
		<td><b>A0</b></td>
		<td>B0</td>
		<td>C0</td>
		<td><b>A1</b></td>
		<td>B1</td>
		<td>C1</td>
		<td><b>A2</b></td>
		<td>B2</td>
		<td>C2</td>
		<td><b>A3</b></td>
		<td>B3</td>
		<td>C3</td>
	</tr>
</table>

Note that arrays of integers are generally subject to both interleaving and integer transformation.

### Referent
Referents are stored as transformed 32-bit signed integers. A value of `-1` (untransformed) indicates a null referent.