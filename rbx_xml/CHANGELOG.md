# rbx_xml Changelog

## Unreleased

## 0.7.0 (2019-05-12)
* Breaking: changed API dramatically to make deserializing instances easier
	* Renamed deserialization APIs to be named like `from_reader`
	* Renamed serialization APIs to be named like `to_writer`
	* Added options to APIs
	* Made error types opaque
* Breaking: changed serialization to be reflection-driven, which improves content compatibility
* Fixed serializing empty `Content` properties ([#146](https://github.com/LPGhatguy/rojo/issues/146))

## 0.6.0 (2019-03-27)
* Added support for new types:
	* `Rect`, `NumberRange`, `NumberSequence`, and `ColorSequence` ([#23](https://github.com/LPGhatguy/rbx-dom/pull/23))
	* `Int64` and `Float64` ([#15](https://github.com/LPGhatguy/rbx-dom/pull/15))
	* `Ref` and `PhysicalProperties`, no longer stubs!
* Rewrote lots of internals, which should improve whitespace handling ([#17](https://github.com/LPGhatguy/rbx-dom/pull/17), [#19](https://github.com/LPGhatguy/rbx-dom/pull/19))

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
* Added support for `Ref` values ([#8](https://github.com/LPGhatguy/rbx-dom/pull/8))

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