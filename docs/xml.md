# Roblox XML Model Format, Version 4
This is unofficial documentation for Roblox's XML model format. The XML model format is used for places (`.rbxlx` files), models (`.rbxmx` files), Roblox Studio settings, and many objects uploaded to Roblox's asset storage.

The XML model format has generally been replaced by the newer, more efficient [binary model format](binary.md). Some use cases for the XML format still exist, owing to its human readability.

This documentation is incomplete. Contributions are welcome.

## Contents
- [File Structure](#file-structure)
- [roblox](#roblox)
- [Meta](#meta)
- [External](#external)
- [Item](#external)
- [Properties](#properties)
- [SharedStrings](#sharedstrings)
	- [SharedString][SharedString-def] (definition)
- [Type Elements](#type-elements)
	- [Axes](#axes)
	- [BinaryString](#binarystring)
	- [BrickColor](#brickcolor)
	- [bool](#bool)
	- [Color3](#color3)
	- [Color3uint8](#color3uint8)
	- [ColorSequence](#colorsequence)
	- [Content](#content)
	- [CoordinateFrame](#coordinateframe)
	- [double](#double) (Float64)
	- [Faces](#faces)
	- [float](#float) (Float32)
	- [Font](#font)
	- [int](#int) (Int32)
	- [int64](#int64)
	- [NumberRange](#numberrange)
	- [NumberSequence](#numbersequence)
	- [OptionalCoordinateFrame](#optionalcoordinateframe)
	- [Ref](#ref) (Referent)
	- [Rect2D](#rect2d)
	- [SharedString][SharedString-use] (property type)
	- [string](#string)
	- [token](#token) (Enum)
	- [UDim](#udim)
	- [UDim2](#udim2)
	- [UniqueId](#uniqueid)
	- [Vector2](#vector2)
	- [Vector2int16](#vector2int16)
	- [Vector3](#vector3)
	- [Vector3int16](#vector3int16)


## File Structure
Roblox XML files consist of a single `<roblox>` element, which contain a sequence of other elements. The basic layout of the file structure is as follows:

- Exactly one `<roblox>` element
	- Zero or more `<Meta>` elements
	- Zero or more `<External>` elements
	- One or more `<Item>` elements
	- One `<Properties>` element
		- Zero or more type elements
	- Zero or more `<Item>` elements (this may nest infinitely)
	- Zero or one `<SharedStrings>` elements
		- Zero or more `<SharedString>` elements

**To be accepted by Roblox, files MUST start with `<roblox` and end with `</roblox>`, with no whitespace.**

## roblox

This element is the root of the file. There MUST be one `roblox` element in the file.

The following attributes are required for this element:

| Name      | Contents                                                                         |
|:----------|:---------------------------------------------------------------------------------|
| `version` | The version of the format this document contains. This MUST be `4` at this time. |

All other attributes are ignored by Roblox, including those defined by Roblox Studio when exporting files.

As stated under [File Structure](#file-structure), this element MUST be the beginning and end of the file.

## Meta

This element represents a single key-value pair of metadata for the file. There MAY be any number of `Meta` elements in a document but they MUST all be under the `roblox` element.

The following attributes are required for this element:

| Name   | Contents                                     |
|:-------|:---------------------------------------------|
| `name` | The key part of the metadata key-value pair. |

The contents of this element represent the value of the metadata key-value pair.

## External

This element is a legacy feature and currently does nothing. Roblox Studio encodes two of these tags when producing files, but they are optional and unused. When present, they MUST be under the `roblox` element.

There are no attributes required for this element.

The contents of this element represent an unknown purpose, as the element does not do anything.

## Item

This element describes one `Instance` value. There SHOULD be at least one of these in all files, as otherwise they serve no purpose, but Roblox accepts files with no `Item` elements. All `Item` elements must be under either the `roblox` element or other `Item` elements.

The following attributes are required for this element:

| Name       | Contents                                                              |
|:-----------|:----------------------------------------------------------------------|
| `class`    | The class of the `Instance` this element represents.                  |
| `referent` | A unique string used to reference this element elsewhere in the file. |

The value of `referent` does not need to follow any pattern, it simply must be unique for the file. Roblox generates referents by prefixing a UUID with `RBX`, but this is not a requirement.

## Properties

This element contains all properties for a given `Instance`. There MUST be one per `Item` element, and each `Properties` element must be under an `Item` element.

There are no attributes required for this element.

Every child of this element is a [Type Element](#type-elements) and represents exactly one property of an `Instance`.

## SharedStrings

This element acts as a repository for `SharedString` definitions. There MAY be zero or one `SharedStrings` element per file. `SharedString` elements must be under the `roblox` element.

There are no attributes required for this element.

## SharedString
[SharedString-def]: #sharedstring

This element defines a single `SharedString` value for reference by [Type Elements](#type-elements). There MAY be zero or more `SharedString` elements per file. `SharedStrings` elements must be under the `SharedStrings` element.

This element shares a name with a type element. That element is documented [here][SharedString-use].

The following attributes are required for this element:

| Name  | Contents                                                               |
|:------|:-----------------------------------------------------------------------|
| `md5` | A unique identifier for this element, for reference by a type element. |

Despite its name, the contents of `md5` do not have to be the MD5 hash of the SharedString and instead simply MUST be a unique identifier for this SharedString.

The value of this element MUST be the SharedString value encoded with Base64.
<!-- TODO: Verify what form of Base64 and put it here-->

## Type Elements

All elements below this point represent a single property on a single `Instance`. They all MUST be under a [`Properties`](#properties) element. There MAY be zero or more of each of these elements under each `Properties` element.

The following attributes are required for all type elements:

| Name   | Contents                                              |
|:-------|:------------------------------------------------------|
| `name` | The nane of the property represented by this element. |

The `name` of a property does not necessarily reflect its in-engine name, and some properties serialize with unexpected names. Additionally, not every property is deserialized by Roblox.

### Axes

### BinaryString

### BrickColor

### bool

### Color3

### Color3uint8

### ColorSequence

### Content

### CoordinateFrame

### double

### Faces

### float

### Font

### int

### int64

### NumberRange

### NumberSequence

### OptionalCoordinateFrame

### Ref

### Rect2D

### SharedString
[SharedString-use]: #sharedstring-1

This element shares a name with a SharedString definition element. That element is documented [here][SharedString-def].

### string

### token

### UDim

### UDim2

### UniqueId

### Vector2

### Vector2int16

### Vector3

### Vector3int16