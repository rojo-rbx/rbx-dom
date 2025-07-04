# rbx_xml Changelog

## Unreleased
* Added support for `Terrain.SmoothGrid`, and adjacent support for `TerrainRegion.SmoothGrid`. ([#444])

[#444]: https://github.com/rojo-rbx/rbx-dom/pull/444

## 1.0.0 (2025-03-28)
* Improved performance of serializer and deserializer by using `Ustr` to represent property and class names ([#462]).
* Implement support for serializing and deserializing the `Content` type. ([#495])
* Updated rbx-dom dependencies, which results in breaking changes to `WeakDom` and `Variant`.

[#462]: https://github.com/rojo-rbx/rbx-dom/pull/462
[#495]: https://github.com/rojo-rbx/rbx-dom/pull/495

## 0.13.5 (2024-08-22)
* Updated rbx-dom dependencies

## 0.13.4 (2024-07-23)
* Updated rbx-dom dependencies

## 0.13.3 (2024-01-16)
* Add the ability to specify a `ReflectionDatabase` to use for serializing and deserializing. This takes the form of `DecodeOptions::reflection_database` and `EncodeOptions::reflection_database`. ([#375])

[#375]: https://github.com/rojo-rbx/rbx-dom/pull/375

## 0.13.2 (2023-10-03)
* Added support for `SecurityCapabilities` values. ([#359])

[#359]: https://github.com/rojo-rbx/rbx-dom/pull/359

## 0.13.1 (2023-08-09)
* Added support for `UniqueId` values. ([#271])
* Added migrations for properties like `ScreenGui.IgnoreGuiInset` and `TextLabel.Font` to their new counterparts (`ScreenGui.GuiInsets` and `TextLabel.FontFace`, respectively). ([#283])
* Added support for `Terrain.MaterialColors`. ([#323])

[#271]: https://github.com/rojo-rbx/rbx-dom/pull/271
[#283]: https://github.com/rojo-rbx/rbx-dom/pull/283
[#323]: https://github.com/rojo-rbx/rbx-dom/pull/323

## 0.13.0 (2023-04-22)
* Added support for `Font` values. ([#248])

[#248]: https://github.com/rojo-rbx/rbx-dom/pull/248

## 0.12.4 (2022-06-12)
* Implemented serialization and deserialization for `Attributes`. ([#219])

[#219]: https://github.com/rojo-rbx/rbx-dom/pull/219

## 0.12.3 (2021-10-11)
* Added support for `Tags` values. ([#199])

[#199]: https://github.com/rojo-rbx/rbx-dom/pull/199

## 0.12.2 (2021-07-19)
* Updated to rbx\_dom\_weak 2.2.
* Added conversion from `Color3` to `Color3uint8` to address [rojo-rbx/rojo#443]. ([#198])

[rojo-rbx/rojo#443]: https://github.com/rojo-rbx/rbx-dom/pull/443
[#198]: https://github.com/rojo-rbx/rojo/issues/198

## 0.12.1 (2021-07-02)
* Upgraded to rbx\_dom\_weak 2.1.

## 0.12.0 (2021-06-26) (yanked)
* Upgraded to rbx\_dom\_weak 2.0 stable.
* Added support for `OptionalCoordinateFrame` type.

## 0.12.0-alpha.4 (2021-04-09)
* rbx_xml now ignore unknown property types. ([#170][pr-170])
	* This fixes our interactions with the new, partially-released `OptionalCoordinateFrame` type that is now showing up in place and model files.

[pr-170]: https://github.com/rojo-rbx/rbx-dom/pull/170

## 0.12.0-alpha.3 (2021-03-08)
* BrickColor properties will now deserialize as the `BrickColor` type.

## 0.12.0-alpha.2 (2021-03-04)
* Fixed support for writing `BrickColor` values.

## 0.12.0-alpha.1 (2021-02-16)
This release is a major, breaking change that upgrades rbx_xml's underlying DOM implementation from rbx_dom_weak 1.0 to 2.0.

* Breaking: upgraded to rbx_dom_weak 2.0
* Added support for the `Ray` type, used in the `RayValue` instance.
* Added support for the `Faces` type, used in the `Handles` instance.
* Added support for the `Axes` type, used in the `ArcHandles` instance.

## 0.11.4 (2019-12-18)
* Updated `base64` dependency

## 0.11.3 (2019-12-11)
* Empty BinaryStrings will now serialize without an empty CDATA tag ([#76](https://github.com/rojo-rbx/rbx-dom/pull/76)).
* Fixed SharedString entries with CRLF line endings causing obscure errors ([#84](https://github.com/rojo-rbx/rbx-dom/pull/84))

## 0.11.2 (2019-08-08)
* Fixed encoding properties in a nondeterministic order. Properties should now be sorted. ([#66](https://github.com/rojo-rbx/rbx-dom/pull/66))

## 0.11.1 (2019-08-01)
* Fixed decoding `BinaryString` values with arbitrary whitespace in them, including CRLF line endings.

## 0.11.0 (2019-07-12)
* Added support for reading and writing `BrickColor` values.

## 0.10.0 (2019-06-10)
* Added support for INF and NAN values in compound types like Vector2 ([#62](https://github.com/rojo-rbx/rbx-dom/pull/62))
* Fixed edge cases around unrecognized Ref properties causing confusing errors.
* Added support for serializing and deserializing `SharedString` values. ([#63](https://github.com/rojo-rbx/rbx-dom/pull/63))

## 0.9.0 (2019-05-16)
* Added real configuration to serialization and deserialization routines
	* Users can now ignore, read/write, or error when encountering unknown properties
	* It's also possible to disable the reflection database entirely, akin to version 0.6.0

## 0.8.0 (2019-05-14)
* Fixed type conversion when serializing properties whose serialized type differs from its canonical type ([#56](https://github.com/rojo-rbx/rbx-dom/pull/56))
* Changed type conversion failures when serializing to elevate to serialization errors ([#58](https://github.com/rojo-rbx/rbx-dom/pull/58))

## 0.7.0 (2019-05-12)
* Breaking: changed API dramatically to make deserializing instances easier
	* Renamed deserialization APIs to be named like `from_reader`
	* Renamed serialization APIs to be named like `to_writer`
	* Added options to APIs
	* Made error types opaque
* Breaking: changed serialization to be reflection-driven, which improves content compatibility
* Fixed serializing empty `Content` properties ([#146](https://github.com/Roblox/rojo/issues/146))

## 0.6.0 (2019-03-27)
* Added support for new types:
	* `Rect`, `NumberRange`, `NumberSequence`, and `ColorSequence` ([#23](https://github.com/rojo-rbx/rbx-dom/pull/23))
	* `Int64` and `Float64` ([#15](https://github.com/rojo-rbx/rbx-dom/pull/15))
	* `Ref` and `PhysicalProperties`, no longer stubs!
* Rewrote lots of internals, which should improve whitespace handling ([#17](https://github.com/rojo-rbx/rbx-dom/pull/17), [#19](https://github.com/rojo-rbx/rbx-dom/pull/19))

## 0.5.0 (2019-03-13)
* Changed referents to be emitted as consecutive integers, which makes generating XML models deterministic regardless of IDs

## 0.4.0 (2019-03-01)
* Updated to `rbx_dom_weak` 1.0
* Fixed `ProtectedString` support, which was missing from the main deserializer loop
* Added support for `Content`, `UDim` and `UDim2` values
* Fixed `Ref` properties pointing to invalid IDs by changing them to always deserialize to `None` for now

## 0.3.0 (2019-02-14)
* Updated `rbx_tree` dependency to `rbx_dom_weak` 0.3.0
* Added support for `ProtectedString`, deserializing as String
* Added support for `Ref` values ([#8](https://github.com/rojo-rbx/rbx-dom/pull/8))

## 0.2.0 (2019-01-25)
* Serialization and deserialization functions are now fully functional
* Added support for new property types:
	* CFrame
	* Color3
	* Color3uint8
	* Enum
	* Float32
	* Int32
	* Vector2
	* Vector2int16
	* Vector3
	* Vector3int16
	* PhysicalProperties (Stub)
* Added support for correctly mapping some property names:
	* FormFactor (serializes as `formFactorRaw`)
	* Size (serializes as `size`)
	* Shape (serializes as `shape`)

## 0.1.0
* Initial release
* Deserialization does not produce instances
* Supports serializing `String` and `Bool` properties
