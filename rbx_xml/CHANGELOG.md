# rbx_xml Changelog

## Unreleased
* Added support for ProtectedString, deserializing as String

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