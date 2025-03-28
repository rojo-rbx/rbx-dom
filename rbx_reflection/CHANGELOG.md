# rbx_reflection Changelog

## Unreleased Changes

## 5.0.0 (2025-03-28)
* Added `ReflectionDatabase::superclasses_iter`
* Added `ReflectionDatabase::has_superclass`, like `Instance:IsA(ClassName)`
* Updated rbx_types to 2.0.0, which includes breaking changes to how a `ReflectionDatabase` serializes

## 4.7.0 (2024-08-22)
* Update to rbx_types 1.10

## 4.6.0 (2024-07-23)
* Update to rbx_types 1.9
* Add `superclasses` method to `ReflectionDatabase` to get a set of superclasses for a given class. ([#402])
* Added method `ReflectionDatabase::find_default_property`, which finds the default value of a property given its name and a class that inherits it. ([#420])

[#402]: https://github.com/rojo-rbx/rbx-dom/pull/402
[#420]: https://github.com/rojo-rbx/rbx-dom/pull/420

## 4.5.0 (2024-01-16)
* Update to rbx_types 1.8.

## 4.4.0
* Update to rbx_types 1.7.

## 4.3.0 (2023-08-09)
* Add support for migrations that should occur from one property to another ([#283])
* Update to rbx_types 1.6.

[#283]: https://github.com/rojo-rbx/rbx-dom/pull/283

## 4.2.0 (2021-10-11)
* Updated to rbx_types 1.3.

## 4.1.0 (2021-07-02)
* Updated to rbx_types 1.1.

## 4.0.0 (2021-06-26) (yanked)
* Updated to rbx_types 1.0.

## 4.0.0-alpha.1 (2021-02-16)
This is a major breaking change as part of the rbx-dom v2 initiative!

* Changed library's role to only contain datatypes, not actual data.
  * Data has moved to the rbx\_reflection_database crate.

## 3.3.418 (2020-02-08)
* Updated reflection database to client 0.418.1.380321

## 3.3.408 (2019-11-05)
* Updated reflection database to client 0.408.0.357216
* Improved accuracy of `scriptability` property on class descriptors.
* Removed default property values for properties only accessible by built-in plugins.
	* This was required by a recent security change in Roblox Studio.

## 3.2.404 (2019-10-04)
* Updated reflection database to client 0.404.0.346082
* Added `Workspace.CollisionGroups` descriptor to improving ergonomics of Rojo.

## 3.2.399 (2019-09-03)
* Updated reflection database to client 0.399.0.334382
* Fixed stack overflow in debug builds without `opt-level=1` caused by reflection database

## 3.2.395
* Updated reflection database to client 0.395.0.324413

## 3.2.390
* Updated reflection database to client 0.390.0.311600
* Improved accuracy of default values. Notably:
	* The distinction between `Float32`, `Float64`, `Int32`, and `Int64` should now be correct.
	* Added defaults for new types like `Rect` and `ColorSequence`

## 3.2.389 (2019-06-12)
* Updated canonical property information for body mover instances like `BodyVelocity`.
* Updated reflection database to client 0.389.1.310791

## 3.1.388 (2019-06-10)
* Updated reflection database to client 0.388.0.307917

## 3.1.384 (2019-05-12)
* Changed getters on descriptor objects to be now marked `#[inline]`
* Added `RbxClassDescriptor::is_service`

## 3.0.384 (2019-05-12)
* Updated reflection database to client 0.384.1.302070
* Added inference for `Content` values from string literals
* Breaking: rewrote reflection database to have a much more conservative public API
* Breaking: introduced the concept of canonical properties and serialization-only properties, which makes handling serialized files much easier.
* Breaking: changed the `ValueResolveError` type to be a struct, which makes it more opaque and less prone to breakage.
* Breaking: renamed version constants to have an `RBX_` prefix.

## 2.0.377 (2019-03-20)
* Updated reflection database to client 0.377.1.289039

## 2.0.374 (2019-03-01)
* Updated to `rbx_dom_weak` 1.0
* Updated reflection database
* Removed default values for some properties like `Parent`
* Added `tags` field (of type `RbxInstanceTags`) to `RbxInstanceClass`

## 1.0.373 (2019-02-26)
* Adjusted version number scheme again to account for patches to the library
* Added `ValueResolveError` to public interface

## 0.2.373 (2019-02-25)
* Adjusted version number to include client release number
* Added default values for serialized properties
* Added version constants
* Added type resolution function, `try_resolve_value`

## 0.1.0 (2019-02-14)
* Initial release
* Exposes a reflection database for instances and enums
* Supports resolving ambiguous `rbx_dom_weak` values
