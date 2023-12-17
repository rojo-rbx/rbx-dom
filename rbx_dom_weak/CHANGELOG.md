# rbx_dom_weak Changelog

## Unreleased Changes
* Implemented `Default` for `WeakDom`, useful when using Serde or creating an empty `WeakDom`

## 2.6.0 (2023-10-03)
* Added `WeakDom::clone_multiple_into_external` that allows cloning multiple subtrees all at once into a given `WeakDom`, useful for preserving `Ref` properties that point across cloned subtrees ([#364])

[#364]: https://github.com/rojo-rbx/rbx-dom/pull/364

## 2.5.0 (2023-08-09)
* Fix potential stack overflow when creating or inserting into a `WeakDom`. ([#279])
* Added `InstanceBuilder::has_property` for checking if an `InstanceBuilder` defines a given property. ([#283])
* Added `WeakDom::clone_within` and `WeakDom::clone_into_external` for cloning instances into the same or a different `WeakDom`, respectively. ([#312])

[#279]: https://github.com/rojo-rbx/rbx-dom/pull/279
[#283]: https://github.com/rojo-rbx/rbx-dom/pull/283
[#312]: https://github.com/rojo-rbx/rbx-dom/pull/312

## 2.4.0 (2022-06-05)
* Added `WeakDom::into_raw` for enabling fast, non-tree-preserving transformations.
* Added `empty`, `with_class`, and `set_class` methods to `InstanceBuilder`.

## 2.3.0 (2021-10-11)
* Updated to rbx_types 1.3.

## 2.2.0 (2021-07-19)
* Updated to rbx_types 1.2.

## 2.1.0 (2021-07-02)
* Updated to rbx_types 1.1.

## 2.0.0 (2021-06-26) (yanked)
* Updated to rbx_types 1.0.

## 2.0.0-alpha.1 (2021-02-16)
This release is a major, breaking change that introduces many fixes and features.

* `RbxTree` was replaced with `WeakDom`
	* Changed `new` to accept an `InstanceBuilder`, which can contain children.
	* Renamed `get_root_id` to `root_ref`.
	* Renamed `get_instance` to `get_by_ref`.
	* Renamed `get_instance_mut` to `get_by_ref_mut`.
	* Renamed `move_instance` to `transfer`.
	* Renamed `set_parent` to `transfer_within`.
	* Renamed `insert_instance` to `insert`. This method now accepts an `InstanceBuilder`.
	* Removed `remove_instance`, replaced by `destroy` and `transfer`.
	* Removed `iter_all_ids`.
	* Removed `descendants`.
	* Added `root` and `root_mut` for accessing the root instance directly.
* `RbxInstanceProperties` was replaced with `InstanceBuilder`
	* This API is completely different, as it's now a builder. It is now much easier to construct instances.
* `RbxInstance` was replaced with `Instance`
	* This type now directly exposes its values instead of implementing `Deref` for another type.
* Moved types into the `rbx_types` crate, re-exported as `rbx_dom_weak::types`.
	* `RbxId` was replaced with `types::Ref`, which can now represent null referents.
* Added `DomViewer` API from rbx_dom_test to make testing instance trees easier.

## 1.10.1 (2019-12-18)
* Updated `base64`, `md5`, and `uuid` dependencies

## 1.10.0 (2019-09-15)
* Added `RbxTree::set_parent`, for moving instances within a single tree.

## 1.9.0 (2019-07-12)
* Changed `BrickColor` to be much more correct
	* All BrickColor values are now correctly available instead of just palette colors
	* Enum values now align with the `BrickColor.Number` property and `BrickColor.new` constructor
	* `BrickColor` no longer serializes to strings in human-readable Serde formats to avoid information loss. These colors have colliding names:
		* Rust
		* Lilac
		* Gold
		* Deep orange

## 1.8.2 (2019-07-10)
* Added conversion from `Int32` to `BrickColor`. This should fix serialization of `SpawnLocation` instances, which use `int` instead of `BrickColor` in at least the XML model format.

## 1.8.1 (2019-06-20)
* Fixed Serde being able to deserialize `RbxValue` and `RbxValueType`'s internal `__Nonexhaustive` value. This should also fix Serde recommending this value to users in error messages.
* Fixed `BrickColor` serialization to use Roblox color names

## 1.8.0 (2019-06-10)
* Added the `SharedString` type, which is used by Roblox to reduce redundant copies of binary buffers. ([#63](https://github.com/rojo-rbx/rbx-dom/pull/63))

## 1.7.0 (2019-05-14)
* Added conversions from `BrickColor` to `Color3` and `Color3uint8`. ([#46](https://github.com/rojo-rbx/rbx-dom/pull/46))
* Added conversions from `Color3` to `Color3uint8` and vice-versa.
* Changed the human readable serialization of `BinaryString` values to be base64-encoded. This makes JSON-encoded values much smaller. This changes the details of the unspecified serialization of rbx_dom_weak.
* `RbxValueConversion` now derives `PartialEq`.

## 1.6.0 (2019-05-12)
* Added `RbxTree::move_instance` API to move instances from one tree to another.
* Fixed `RbxTree::descendants` to no longer return the instance of the ID you give it. This may break code relying on this (broken) assumption, but was definitely a bug.
* `RbxValueConversion` now derives `Debug` and `Clone` ([#52](https://github.com/rojo-rbx/rbx-dom/issues/52))

## 1.5.0 (2019-05-06)
* Added support for `BrickColor` ([#29](https://github.com/rojo-rbx/rbx-dom/pull/29))
* Added `RbxValue::try_convert_ref`, which can be used to try to coerce values. ([#42](https://github.com/rojo-rbx/rbx-dom/pull/42))
* `Content` values can now be inferred from string literals.
* `RbxValueType` now derives Serde's `Serialize` and `Deserialize` traits

## 1.4.0 (2019-03-27)
* Added support for new types:
	* `NumberRange`
	* [#15](https://github.com/rojo-rbx/rbx-dom/pull/15):
		* `Int64`
		* `Float64`
	* [#20](https://github.com/rojo-rbx/rbx-dom/pull/20):
		* `Rect`
		* `Ray`
		* `ColorSequence`
		* `NumberSequence`

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
* Added support for `Ref` values ([#8](https://github.com/rojo-rbx/rbx-dom/pull/8))
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
