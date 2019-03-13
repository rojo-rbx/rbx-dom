# rbx_xml Changelog

## Unreleased
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