# rbx\_dom\_weak Changelog

## Unreleased
* Added `Int64` and `Float64` types to `RbxValue` ([#15](https://github.com/LPGhatguy/rbx-dom/pull/15))

## 1.3.0 (2019-03-14)
* Added `sort_children_by_key` and `sort_children_unstable_by_key` methods to `RbxInstance` to reorder children safely

## 1.2.0 (2019-03-13)
* `RbxTree` and `RbxInstance` are now clonable without an `unimplemented` panic
	* Cloned trees preserve their IDs as-is and there's no public API yet to transplant instances between trees. This is mostly useful for comparing trees before/after a mutation, which we're using in Rojo.

## 1.1.0 (2019-03-11)
* Marked `PhysicalProperties` as `pub`
* Fixed `Serialize` impl for `UnresolvedRbxValue`
	* This removes `Serialize` from `AmbiguousRbxValue`, which should not break any real code.

## 1.0.0 (2019-03-01)
* `RbxValue` and `RbxValueType` can no longer be matched exhaustively, which enables adding new types without breaking code in the future
* Added support for `UDim`, `UDim2`, and `Content` value types
* `PhysicalProperties` is no longer a stub type

## 0.3.1 (2019-02-26)
* Added support for bare bool values when deserializing `UnresolvedRbxValue`
* Implemented `Serialize` for `UnresolvedRbxValue` and `AmbiguousRbxValue`
* Implemented `From<RbxValue>` for `UnresolvedRbxValue`

## 0.3.0 (2019-02-14)
* Renamed crate from `rbx_tree` to `rbx_dom_weak`
* Added support for `Ref` values ([#8](https://github.com/LPGhatguy/rbx-dom/pull/8))
* Added `UnresolvedRbxValue` and `AmbiguousRbxValue`, intended to be used alongside `rbx_reflection` to make specifying values less verbose.

## 0.2.0 (2019-01-25)
* Added new variants for `RbxValue`:
	* Int32
	* Float32
	* Enum
	* Vector2
	* Color3
	* Color3uint8
	* Vector3int16
	* Vector2int16
	* CFrame
	* PhysicalProperties (Stub)

## 0.1.0
* Initial release
* Supports `String`, `Bool`, and `Vector3` property values