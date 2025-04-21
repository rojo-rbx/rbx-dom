# rbx_binary Changelog

## Unreleased

## 1.0.0 (2025-03-28)
* Dramatically improved performance of serializer and deserializer by using `Ustr` to represent property and class names ([#462]).
* Added the ability to specify what type of compression to use for serializing. This takes the form of `Serializer::compression_type`. ([#446])
* Added support for ZSTD compressed files ([#446])
* Implicit lossy conversion of non-UTF-8 `Instance.Name` and `*Script.Source` properties when decoding.  The previous behaviour was returning an error. ([#380])
* Implement support for serializing and deserializing the `Content` type. ([#495])
* Updated rbx-dom dependencies, which results in breaking changes to `WeakDom` and `Variant`.

[#462]: https://github.com/rojo-rbx/rbx-dom/pull/462
[#446]: https://github.com/rojo-rbx/rbx-dom/pull/446
[#380]: https://github.com/rojo-rbx/rbx-dom/pull/380
[#495]: https://github.com/rojo-rbx/rbx-dom/pull/495

## 0.7.7 (2024-08-22)
* Updated rbx-dom dependencies

## 0.7.6 (2024-08-06)
* Changed the way instances are added to the serializer to a depth-first post-order traversal. ([#432])

[#432]: https://github.com/rojo-rbx/rbx-dom/pull/432

## 0.7.5 (2024-07-23)
* Within `PRNT` chunks, parent-child links are now generated depth-first so that parents always come after their children in the chunk. ([#411])

[#411]: https://github.com/rojo-rbx/rbx-dom/pull/411

## 0.7.4 (2024-01-16)
* Add the ability to specify a `ReflectionDatabase` to use for serializing and deserializing. This takes the form of `Deserializer::reflection_database` and `Serializer::reflection_database`. ([#375])

[#375]: https://github.com/rojo-rbx/rbx-dom/pull/375

## 0.7.3 (2023-10-23)
* Fixed missing fallback default for `SecurityCapabilities` ([#371]).

## 0.7.2 (2023-10-03)
* Added support for `SecurityCapabilities` values. ([#361])
* Fixed potential crash when serializing `SharedString` values ([#363])

[#361]: https://github.com/rojo-rbx/rbx-dom/pull/361
[#363]: https://github.com/rojo-rbx/rbx-dom/pull/363

## 0.7.1 (2023-08-09)
* Added support for `UniqueId` values. ([#271])
* Added migrations for properties like `ScreenGui.IgnoreGuiInset` and `TextLabel.Font` to their new counterparts (`ScreenGui.GuiInsets` and `TextLabel.FontFace`, respectively). ([#283])
* Added support for `Terrain.MaterialColors`. ([#323])

[#271]: https://github.com/rojo-rbx/rbx-dom/pull/271
[#283]: https://github.com/rojo-rbx/rbx-dom/pull/283
[#323]: https://github.com/rojo-rbx/rbx-dom/pull/323

## 0.7.0 (2023-04-22)
* Added support for `Font` values. ([#248])
* Fixed the nondeterministic output of SSTR chunk when multiple shared strings are present. ([#254])

[#248]: https://github.com/rojo-rbx/rbx-dom/pull/248
[#254]: https://github.com/rojo-rbx/rbx-dom/pull/254

## 0.6.6 (2022-06-29)
* Fixed unserialized properties getting deserialized, like `BasePart.MaterialVariant`. ([#230])

[#230]: https://github.com/rojo-rbx/rbx-dom/pull/230

## 0.6.5 (2022-06-12)
* Implemented serialization and deserialization of `Attributes`. ([#220])

[#220]: https://github.com/rojo-rbx/rbx-dom/pull/220

## 0.6.4 (2021-10-11)
* Fixed writing models where some instances have `Tags` properties and others do not.

## 0.6.3 (2021-10-11)
* Added support for `Tags` values. ([#199])

[#199]: https://github.com/rojo-rbx/rbx-dom/pull/199

## 0.6.2 (2021-07-19)
* Upgraded to rbx\_dom\_weak 2.2.

## 0.6.1 (2021-07-02)
* Upgraded to rbx\_dom\_weak 2.1.

## 0.6.0 (2021-06-26) (yanked)
* Upgraded to rbx\_dom\_weak 2.0 stable.
* Fixed null Refs not being written for unserialized instances. ([#184][pr-184])
* Changed interface:
  * Renamed `from_reader_default` to `from_reader`.
  * Added `Deserializer` type.
  * Renamed `from_writer_default` to `from_writer` and reordered arguments.
  * Added `Serializer` type.

[pr-184]: https://github.com/rojo-rbx/rbx-dom/pull/184

## 0.6.0-alpha.5 (2021-05-14)
* Added `OptionalCoordinateFrame` support. ([#176][pr-176])
* Added support for writing CFrame basic rotation IDs. ([#171][pr-171])

[pr-171]: https://github.com/rojo-rbx/rbx-dom/pull/171
[pr-176]: https://github.com/rojo-rbx/rbx-dom/pull/176

## 0.6.0-alpha.4 (2021-04-09)
* Unknown property types will no longer fail deserialization ([#168][pr-168])
    * This makes rbx_binary more resilient to new types being added by Roblox.
* `PROP` chunks that end after their prop name are now silently ignored. ([#168][pr-168])
    * This matches Roblox's behavior and fixes our handling of `OptionalCoordinateFrame`, added in the most recent Roblox Studio release.

[pr-168]: https://github.com/rojo-rbx/rbx-dom/pull/168

## 0.6.0-alpha.3 (2021-03-08)
* Added support for encoding `Int32` values as `BrickColor`.
  * This helps improve compatibility when decoding XML models and reencoding them as binary.

## 0.6.0-alpha.2 (2021-03-04)
* Added support for widening integer types when serializing.
    * `Int32` will widen to `Int64`, `Float32` will widen to `Float64`.

## 0.6.0-alpha.1 (2021-02-16)
This release is a major, breaking change that upgrades rbx\_xml's underlying DOM implementation from rbx\_dom\_weak 1.0 to 2.0. This release also realigned rbx\_binary's API to match rbx_xml.

* Breaking: ported crate to rbx\_dom_weak 2.0
* Breaking: updated top-level API to match rbx_xml.
* Added support for all remaining types.

## 0.5.0 (2019-12-18)
0.5.0 is intended to be mostly API-compatible with previous rbx_binary releases because it makes a lot of foundational changes. 0.6.0 will break the rbx_binary API significantly.

* Rewrote crate from ground-up using understanding gained from rbx_xml
* Added support for using reflection information, improving content compatibility
* Improved performance by up to 30% for some files
* Improved instrumentation using the `log` crate
* Improved the crate's error types ([#48](https://github.com/rojo-rbx/rbx-dom/issues/48))
* Fixed panics in many cases, instead returning an error ([#26](https://github.com/rojo-rbx/rbx-dom/issues/26))
* Fixed handling unknown `BinaryString` values ([#49](https://github.com/rojo-rbx/rbx-dom/issues/49))

## 0.4.1 (2019-05-29)
* Fixed bad interaction with rbx_dom_weak 1.6.0 causing instances to go missing.

## 0.4.0 (2019-03-01)
* Updated to `rbx_dom_weak` 1.0

## 0.3.0 (2019-02-14)
* Updated `rbx_tree` dependency to `rbx_dom_weak` 0.3.0

## 0.2.0 (2019-01-25)
* Updated `rbx_tree` dependency to 0.2.0

## 0.1.0
* Initial release
* Supports `String` and `Bool` types
