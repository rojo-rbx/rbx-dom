# rbx_types Changelog

## Unreleased Changes
* Implement `IntoIterator` for `&Attributes`. ([#386])
* Implement `Extend<(String, Variant)>` for `Attributes`. ([#386])

[#386]: https://github.com/rojo-rbx/rbx-dom/pull/386

## 1.8.0 (2024-01-16)
* Add `len` and `is_empty` methods to `Attributes` struct. ([#377])

[#377]: https://github.com/rojo-rbx/rbx-dom/pull/377

## 1.7.0 (2023-10-03)
* Implemented `FromStr` for `TerrainMaterials`. ([#354])
* `MaterialColorsError` and `UniqueIdError` are no longer publicly exposed. ([#355])
* Implemented barebones `SecurityCapabilities`. ([#358])
* Implement `Display` for `SharedStringHash` ([#363])

[#354]: https://github.com/rojo-rbx/rbx-dom/pull/354
[#355]: https://github.com/rojo-rbx/rbx-dom/pull/355
[#358]: https://github.com/rojo-rbx/rbx-dom/pull/358
[#363]: https://github.com/rojo-rbx/rbx-dom/pull/363

## 1.6.0 (2023-08-09)
* Added support for `UniqueId` values. ([#271])
* Changed `BinaryString`'s non-human readable serde implementation to be identical to `Vec<u8>`. ([#276])
* Added `Font::new` and `Font::regular` constructors. ([#283])
* Added support for `CFrame` values in attributes. ([#296])
* Added support for `Font` values in attributes. ([#299])
* Implemented `MaterialColors`. ([#323])

[#271]: https://github.com/rojo-rbx/rbx-dom/pull/271
[#276]: https://github.com/rojo-rbx/rbx-dom/pull/276
[#283]: https://github.com/rojo-rbx/rbx-dom/pull/283
[#296]: https://github.com/rojo-rbx/rbx-dom/pull/296
[#299]: https://github.com/rojo-rbx/rbx-dom/pull/299
[#323]: https://github.com/rojo-rbx/rbx-dom/pull/323

## 1.5.0 (2023-04-22)
* Implemented `Font`. ([#248])

[#248]: https://github.com/rojo-rbx/rbx-dom/pull/248

## 1.4.2 (2022-06-12)
* `Variant::String` now encodes correctly inside of `Attributes`.

## 1.4.1 (2022-06-12)
* `Attributes` can now decode from an empty buffer.
* `Attributes` now encodes as an empty buffer when empty.

## 1.4.0 (2022-06-05)
* Added `Attributes::with` for creating attributes in code more easily.
* Implemented `Hash` for `VariantTy`.

## 1.3.0 (2021-10-11)
* Implemented `Tags`. ([#199])
* Implemented `Attributes`. ([#166])

[#166]: https://github.com/rojo-rbx/rbx-dom/pull/166
[#199]: https://github.com/rojo-rbx/rbx-dom/pull/199

## 1.2.0 (2021-07-19)
* Implemented `From<Color3>` for `Color3uint8` and `From<Color3uint8>` for `Color3`. ([#198][#198])

[#198]: https://github.com/rojo-rbx/rbx-dom/pull/198

## 1.1.0 (2021-07-02)
* Critical fix: changed serde serialization of fields from PascalCase to camelCase. ([#191][#191])

[#191]: https://github.com/rojo-rbx/rbx-dom/pull/191

## 1.0.0 (2021-06-26) (yanked)
* Removed `legacy-compact` feature, which added conversions from rbx\_dom\_weak 1.x types.

## 0.4.0 (2021-06-26)
* Changed `Variant` serialization to use Serde's default enum representation.
  * Before: `{ "type": "Vector2", "value": [1.0, 2.0] }`
  * After: `{ "Vector2": [1.0, 2.0] }`
* Implemented `PartialOrd` and `Ord` for `SharedStringHash`.

## 0.3.1 (2021-05-14)
* Added `Variant::OptionalCoordinateFrame`.

## 0.3.0 (2021-02-16)
* Renamed `EnumValue` to `Enum`.
* Added `Display` and `FromStr` implementations for `Ref`.

## 0.2.0 (2020-04-27)
* `Ref` can now represent null explicitly via `Ref::none` and `Ref::is_none`.
* Added `Region3` and `Region3int16` to `Variant` and `VariantType`.
* Added `legacy-compat` feature to enable conversions with rbx_dom_weak 1.x.

## 0.1.0 (2020-02-05)
* Initial release of types crate, should have rough feature parity with rbx_dom_weak.
* API will move a lot before becoming stable.
