# Roblox XML Model Format, Version 4
This is unofficial documentation for Roblox's XML model format. The XML model format is used for places (`.rbxlx` files), models (`.rbxmx` files), Roblox Studio settings, and many objects uploaded to Roblox's asset storage.

The XML model format has generally been replaced by the newer, more efficient [binary model format](binary.md). Some use cases for the XML format still exist, owing to its human readability.

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
	- [bool](#bool)
	- [BrickColor](#brickcolor)
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
	- [Optional](#optional)
	- [PhysicalProperties](#physicalproperties)
	- [ProtectedString](#protectedstring)
	- [Ray](#ray)
	- [Rect2D](#rect2d)
	- [Ref](#ref) (Referent)
	- [SharedString][SharedString-use] (property type)
	- [string](#string)
	- [token](#token) (Enum)
	- [UDim](#udim)
	- [UDim2](#udim2)
	- [UniqueId](#uniqueid)
	- [Vector2](#vector2)
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

All properties are encoded as a single element parented under a `Properties` element. Each element represents exactly one property for one `Instance`.

The contents of the element vary depending upon the type of the property it represents. The name of the element SHOULD be the name of the datatype, but Roblox does not require it to be.

The following attributes are required for all property elements, regardless of their type or name:


| Name   | Contents                                              |
|:-------|:------------------------------------------------------|
| `name` | The nane of the property represented by this element. |

The `name` of a property does not necessarily reflect its in-engine name, and some properties serialize with unexpected names. Additionally, not every property is deserialized by Roblox.

The format for each data type is listed below.

### Axes

The `Axes` data type is represented with a single `axes` element that contains a single integer between `0` and `7`. This integer represents a bitfield of the `Z`, `Y`, and `X` axes packed into the lower 3 bits of it, in that order.

An `Axes` property with only the `X` axis enabled would appear as follows:

```xml
<Axes name="AxesExample">
	<axes>1</axes>
</Axes>
```

### BinaryString

The `BinaryString` data type is represented by the contents of the property encoded with Base64.

A `BinaryString` property with the contents `Rojo is cool!` would appear as follows:

```xml
<BinaryString name="BinaryStringExample">Um9qbyBpcyBjb29sIQ==</BinaryString>
```

### bool

The `bool` data type is represented by a literal string reading either `true` or `false` depending upon the state of the value.

Although Roblox accepts variations such as `fAlSe` and `TRUE`, by convention values SHOULD be written in all lowercase.

A `bool` with the value `false` would appear as follows:

```xml
<bool name="BoolExample">false</bool>
```

### BrickColor

The `BrickColor` data type is represented by a single 32-bit integer that represents the `Number` of the value.

Roblox encodes this type with the element name `int` but by convention, the element SHOULD be named `BrickColor`. Either is accepted.

A `BrickColor` with the value `Medium Stone Grey` (whose number is `194`) should appear as follows:

```xml
<BrickColor name="BrickColorExample">194</BrickColor>
```

### Color3

The `Color3` data type is represented by three child elements named `R`, `G`, and `B`. These elements contain the value of that component as 32-bit floating point numbers. See [`float`](#float) for more information on the format of floating point numbers.

A `Color3` with the value `INF, 1337, 0.15625` would appear as follows:

```xml
<Color3 name="Color3Example">
	<R>INF</R>
	<G>1337</G>
	<B>0.15625</B>
</Color3>
```

### Color3uint8

The `Color3uint8` data type is represented by a single unsigned 32-bit integer that is the `R`, `G`, and `B` components of the color (as integers in the range 0 to 255) packed into the lower 24 bits of the number. This integer is little-endian and written in the order `G`, `B`, `R`.

Roblox encodes this type with the upper 8 bits filled with `FF` (in hexadecimal). This SHOULD be done by encoders to avoid compatibility issues.

A `Color3uint8` with the value `96, 64, 32` would appear as follows:

```xml
<Color3uint8 name="Color3uint8Example">4284497952</Color3uint8>
```

### ColorSequence

The `ColorSequence` data type is represented by a series of floating-point numbers seperated by a single space. Every 5 elements in this series represents a single keypoint of the `ColorSequence`. The elements are written in the order `Time`, `Value.R`, `Value.G`, `Value.B`, and `Envelope`.

At this moment, the `Envelope` section of this sequence is unused and SHOULD always be `0`. It MUST be included.

`ColorSequence` values MUST have one keypoint with the `Time` field set to `0` and MUST have one keypoint with the `Time` field set to `1`.

A `ColorSequence` with the value `[0, 96, 64, 32] [1, 5, 10, 15]` would appear as follows:

```xml
<ColorSequence name="ColorSequenceExample">0 0.376471 0.25098 0.12549 0 1 0.0196078 0.0392157 0.0588235 0 </ColorSequence>
```

### Content

The `Content` data type is represented by a single element with one of several child elements. Currently, the name of this child element may be `url` or `null`. Historically, it could be `binary` or `hash`. This child element is not nillable and MUST include an opening and closing tag.

If the child element is `url`, then the value of it is the `Content`'s URI. If the element is `null`, it indicates the `Content` is empty. When the child element is `null`, it MUST be empty. 

If the child element is either `binary` or `hash`, the contents SHOULD be disregarded and the `Content` should be viewed as empty. These tags MUST NOT be written by encoders.

A `Content` with the value `rbxasset://textures/face.png` would appear as follows:

```xml
<Content name="ContentExample"><url>rbxasset://textures/face.png</url></Content>
```

Additionally, a `Content` with no value would appear as follows:

```xml
<Content name="ContentExample"><null></null></Content>
```

### CoordinateFrame

The `CFrame` data type is represented by a single element named `CoordinateFrame` with 12 child elements representing each of the components of the value. In order, these components are: `X`, `Y`, `Z`, `R00`, `R01`, `R02`, `R10`, `R11`, `R12`, `R20`, `R21`, `R22`. Each of these child elements is a [`float`](#float) value.

Despite the canonical name of the data type being `CFrame`, elements of this type MUST be named `CoordinateFrame` to maintain compatibility.

A `CFrame` with the components `0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1` would appear as follows:

```xml
<CoordinateFrame name="CoordinateFrameExample">
	<X>0</X>
	<Y>0</Y>
	<Z>0</Z>
	<R00>1</R00>
	<R01>0</R01>
	<R02>0</R02>
	<R10>0</R10>
	<R11>1</R11>
	<R12>0</R12>
	<R20>0</R20>
	<R21>0</R21>
	<R22>1</R22>
</CoordinateFrame>
```

### double

The `double` data type (also known as `Float64`) is represented as a standard 64-bit floating point number would be. Specifically, it is represented by the [XSD precision decimal](https://www.w3.org/TR/xsd-precisionDecimal/) type. For full details, view the XSD specification but strings such as `1.0`, `1`, `-0`, and `13e37` are all valid representations of `double` values.

Positive infinity is represented as `INF` or `+INF`, negative infinity is represented as `-INF`, and NaN is represented as `NAN`. To be compatible, encoders MUST use these representations, including the all upper casing.

Encoders SHOULD encode `double` values with at least 17 digits of precision but they MAY elect to use less depending upon the property and their own needs.

A `double` with the value `0.15625` would appear as follows:

```xml
<double name="DoubleExample">0.15625</double>
```

### Faces

The `Faces` data type is represented with a single `faces` element that contains a single integer between `0` and `63`, inclusive. This integer represents a bitfield of the `Right`, `Top`, `Back`, `Left`, `Bottom`, and `Front` faces packed into the lower 6 bits of it, in that order.

A `Faces` property with the `Front`, `Left`, and `Top` faces enabled would appear as follows:

```xml
<Faces name="FacesExample">
	<faces>42</faces>
</Faces>
```

### float

The `float` data type (also known as `Float32` or `single`) is represented as a standard 32-bit floating-point number would be. Specifically, it is represented by the [XSD precision decimal](https://www.w3.org/TR/xsd-precisionDecimal/) type. For full details, view the XSD specification, but strings such as `1.0`, `1`, `-0`, and `13e37` are all valid representations of `float` values.

Positive infinity is represented as `INF` or `+INF`, negative infinity is represented as `-INF`, and NaN is represented as `NAN`. To be compatible, encoders MUST use these representations, including the all upper casing.

Encoders SHOULD encode `float` values with at least 9 digits of precision but they MAY elect to use less depending upon the property and their own needs.

A `float` with the value `0.15625` would appear as follows:

```xml
<float name="FloatExample">0.15625</float>
```

### Font

The `Font` data type is represented with 4 child elements. These elements and their type is listed as follows:

- `Family` - `Content`
- `Weight` - `int`
- `Style` - `String`
- `CachedFaceId` - `Content`

The `Family` element is a URI to the family definition of the value. This will likely be a local file.

The `Weight` element is a value of an item from the Roblox `FontWeight` enum. This will be a value in between `100` and `900` (inclusive) that is a multiple of `100`.

The `Style` element is the name of an item from the Roblox `FontStyle` enum. At this time, the only values are `Normal` and `Italic`.

The `CachedFaceId` element will point to a locally cached copy of the `Font`'s source file if it is present.

A `Font` with the value `Arial, Italic, Bold` would appear as follows:

```xml
<Font name="FontExample">
	<Family><url>rbxasset://fonts/families/Arial.json</url></Family>
	<Weight>700</Weight>
	<Style>Italic</Style>
</Font>
```

### int

The `int` data type (also known as `Int32`) is represented as a number in the range `-2147483648` to `2147483647`, inclusive. This is the range of a signed 32-bit integer.

Positive numbers MUST NOT be prefixed with `+`.

An `int` value of `1337` would appear as follows:

```xml
<int name="IntExample">1337</int>
```

### int64

The `int64` data type (also known as `Int64` or `long`) is represented as a number in the range `-9223372036854775808` to `9223372036854775807`, inclusive. This is the range of a 64-bit integer.

Positive numbers MUST NOT be prefixed with `+`.

An `int64` value of `-559038737` would appear as follows:

```xml
<int64 name="Int64Example">-559038737</int64>
```

### NumberRange

The `NumberRange` data type is represented as sequence of two floating-point numbers seperated by a space. These numbers represent the `Min` and `Max` components of the value in that order.

Both numbers are formatted as [`float`](#float) values.

A `NumberRange` value of `0.15625, 1337` would appear as follows:

```xml
<NumberRange name="NumberRangeExample">0.15625 1337 </NumberRange>
```

### NumberSequence

The `NumberSequence` data type is represented by a series of floating-point numbers seperated by a single space. Every 3 elements in this series represents a single keypoint of the `NumberSequence`. The elements are written in the order `Time`, `Value`, and `Envelope`.

`NumberSequence` values MUST have one keypoint with the `Time` field set to `0` and MUST have one keypoint with the `Time` field set to `1`.

A `NumberSequence` with the value `[0, 6, 3] [1, 4, 2]` would appear as follows:

```xml
<NumberSequence name="NumberSequenceExample">0 6 3 1 4 2 </NumberSequence>
```

### Optional

The `Optional<T>` data type represents an optional value of type `T` and is represented by an element with either one or zero child elements. If the value is present, there will be a child element of type `T`. Otherwise, there is no child element.

Elements of this type should be named `Optional` followed by the name of the type. As an example, for `Optional<CoordinateFrame>`, the element should be named `OptionalCoordinateFrame`.

The name of the child element varies depending upon the type `T` is. The following is a list ofcurrently valid types for `T`, along with the name of the child element:

| Type                                  | Child Element Name |
|:--------------------------------------|:-------------------|
| [`CoordinateFrame`](#coordinateframe) | `CFrame`           |

An `Optional<CoordinateFrame>` where the value was `0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1` would appear as follows:

```xml
<OptionalCoordinateFrame name="OptionalExample">
	<CFrame>
		<X>0</X>
		<Y>0</Y>
		<Z>0</Z>
		<R00>1</R00>
		<R01>0</R01>
		<R02>0</R02>
		<R10>0</R10>
		<R11>1</R11>
		<R12>0</R12>
		<R20>0</R20>
		<R21>0</R21>
		<R22>1</R22>
	</CFrame>
</OptionalCoordinateFrame>
```


### PhysicalProperties

The `PhysicalProperties` data type is represented as a sequence of either one or six child elements. The first child element is named `CustomPhysics` and is a [`bool`](#bool) value indicating whether the data type is custom or not.

If `CustomPhysics` is `true`, then there will be an additional `5` child elements. They are named `Density`, `Friction`, `Elasticity`, `FrictionWeight`, and `ElasticityWeight` and represent the respective components of the value. Each of these child elements is a [`float`](#float) value.

If `CustomPhysics` is `false`, then it will be the only child element present.

A custom `PhysicalProperties` created with this constructor:

```lua
PhysicalProperties.new(1, 2, 3, 0.15625, 1.25)
```

Would appear as follows:

```xml
<PhysicalProperties name="PhysicalPropertiesExample">
	<CustomPhysics>true</CustomPhysics>
	<Density>1</Density>
	<Friction>2</Friction>
	<Elasticity>1</Elasticity>
	<FrictionWeight>0.15625</FrictionWeight>
	<ElasticityWeight>1.25</ElasticityWeight>
</PhysicalProperties>
```

### ProtectedString

The `ProtectedString` data type is represented as a string. This data type MUST have its contents maintained exactly. Whitespace MUST be preserved for this type.

To ease use, `ProtectedString` values SHOULD have their contents written as surrounded by `CDATA`. If this is not possible, then care must be taken to escape characters when necessary.

A `ProtectedString` with the contents `print("Hello, world!")` message would appear as follows:

```xml
<ProtectedString name="ProtectedStringExample"><![CDATA[print("Hello world!")]]></ProtectedString>
```

### Ray

The `Ray` data type is represented as a sequence of two child elements representing the `Origin` and `Direction` components of the value. These child elements are named `origin` and `direction` and are both [`Vector3`](#vector3) values.

A `Ray` with the value `[<1, 2, 3>, <-1, -2, -3>]` would appear as follows:

```xml
<Ray name="RayExample">
	<origin>
		<X>1</X>
		<Y>2</Y>
		<Z>3</Z>
	</origin>
	<direction>
		<X>-1</X>
		<Y>-2</Y>
		<Z>-3</Z>
	</direction>
</Ray>
```

### Rect2D

The `Rect2D` data type (also known as `Rect`) is represented as a sequence of two child elements representing the `Min` nad `Max` components of the value. These child elements are named `min` and `max` and are both [`Vector2`](#vector2) values.

Despite the canonical name of the data type being `Rect`, elements of this type SHOULD be named `Rect` to maintain compatibility.

A `Rect2D` with the value `[<1, 2>, <3, 4>)` would appear as follows:

```xml
<Rect2D name="Rect2DExample">
	<min>
		<X>1</X>
		<Y>2</Y>
	</min>
	<max>
		<X>3</X>
		<Y>4</Y>
	</max>
</Rect2D>
```

### Ref

The `Ref` data type (also known as `Referent`) is represented by a literal string that corresponds to the `referent` attribute of an [`Item`](#item) element. The `Item` element that this `referent` belongs to represents the `Instance` pointed to by.

Roblox encodes empty `Ref` values as `null`. Encoders SHOULD also use `null` to refer to an empty `Ref` value when necessary.

Although the canonical name of this data type is `Referent`, elements of this type MUST be named `Ref` to ensure compatibility.

A `Ref` value pointing to a random `Item` may appear as follows:

```xml
<Ref name="Example">RBX466F72207262782D646F6D21203A2D29</Ref>
```

### SharedString
[SharedString-use]: #sharedstring-1

This element shares a name with a SharedString definition element. That element is documented [here][SharedString-def].

The `SharedString` data type is represented by a string that points to a `SharedString` defined elsewhere in the file. Specifically, the contents of elements of this type should be equal to the `md5` attribute of a [`SharedString` definition][#SharedString-def].

A `SharedString` value may appear as follows:

```xml
<SharedString name="SharedStringExample">ZGVra29ub3Rfd2FzX2hlcmU=</SharedString>
```

### string

The `string` data type is represented as a literal sequence of characters inside an element.

Proper care must be taken to escape characters when necessary.

A `string` value `Hello, world!` would appear as follows:

```xml
<string name="StringExample">Hello, world!</string>
```

### token

The `token` data type (also known as `Enum`) is represented as a sequence of numbers indicating the underlying `Value` of the enum.

Despite the canonical name of the data type being `Enum`, elements of this type MUST be named `token` to maintain compatibility.

A `token` representing `Enum.NormalId.Left` would appear as follows:

```xml
<token name="TokenExample">3</token>
```

### UDim

The `UDim` data type is represented as a sequence of two child elements indicating the `Scale` and `Offset` components of the value. These child elements are named `S` and `O`. The `S` element is a [`float`](#float) and the `O` element is a [`int`](#int).

A `UDim` with the value `{0.15625, 1337}` would appear as follows:

```xml
<UDim name="UDimExample">
	<S>0.15625</S>
	<O>1337</O>
</UDim>
```

### UDim2

The `UDim2` data type is represented as a sequence of four child elements indicating the `X.Scale`, `X.Offset`, `Y.Scale`, `Y.Offset` components of the value. These elements are named `XS`, `XO`, `YS`, and `YO`. The `XS` and `YS` elements are [`float`](#float) values and the `XO` and `YO` elements are [`int`](#int) values.

A `UDim2` with the value `{0.15625, 1337}, {-123, 456}` would appear as follows:

```xml
<UDim2 name="UDim2Example">
	<XS>0.15625</XS>
	<XO>1337</XO>
	<YS>-123</YS>
	<YO>456</YO>
</UDim2>
```

### UniqueId

The `UniqueId` data type is represented as hexadecimal-encoded sequence of `16` bytes. These bytes may be split in three distinct groups, representing components of the `UniqueId`:

| Range   | Component Name | Format                  |
|:--------|:---------------|:------------------------|
|  0 -  7 | Random         | Unsigned 64-bit integer |
|  8 - 11 | Time           | Unsigned 32-bit integer |
| 12 - 15 | Index          | Unsigned 32-bit integer |

**NOTE**: The `Random` component is serialized differently between the XML and [binary](binary.md) format. Specifically, in the XML format it is left-circular rotated by `1` bit. Care MUST be taken to ensure equivalent values are modified to be correctly equivalent when reading and writing between formats.

A `UniqueId` may appear as follows:

```xml
<UniqueId name="UniqueIdExample">686f6c792062696e676c6521203a33</UniqueId>
```

### Vector2

The `Vector2` data type is represented as a sequence of two child elements. These child elements are named `X` and `Y` and represent the respective components of the value. Both of these elements are [`float`](#float) values.

A `Vector2` with the value `<Infinity, 1337>` would appear as follows:

```xml
<Vector2 name="Vector2Example">
	<X>INF</X>
	<Y>1337</Y>
</Vector2>
```

### Vector3

The `Vector3` data type is represented as a sequence of three child elements. These child elements are named `X`, `Y`, and `Z` and represent the respective components of the value. All three elements are [`float`](#float) values.

A `Vector3` with the value `<-Infinity, 0.15625, -1337>` would appear as follows:

```xml
<Vector3 name="Vector3Example">
	<X>-INF</X>
	<Y>0.15625</Y>
	<Z>-1337</Z>
</Vector3>
```

### Vector3int16

The `Vector3int16` data type is represented as a sequence of three child elements. These child elements are named `X`, `Y`, and `Z` and represent the respective components of the value.

All three child elements MUST be in the range `-32768` to `32767`, inclusive.

A `Vector3int16` with the value `<1337, 0, -1337>` would appear as follows:

```xml
<Vector3int16 name="Vector3int16Example">
	<X>1337</X>
	<Y>0</Y>
	<Z>-1337</Z>
</Vector3int16>
```