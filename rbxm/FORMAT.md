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

## File Structure
1. File Header
2. Chunks
	1. Zero or one `META` chunks
	2. `INST` chunk
	3. Zero or more `PROP` chunks
	4. `PRNT` chunk
	5. `END` chunk

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
TODO

### `PROP` Chunk
TODO

### `PRNT` Chunk
TODO

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