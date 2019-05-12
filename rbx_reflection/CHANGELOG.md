# rbx_reflection Changelog

## [Unreleased]
* `Content` values can now be inferred from string literals
* Rewrote reflection database to have a much more conservative public API
* Introduced the concept of canonical properties and serialization-only properties, which makes handling serialized files much easier.
* Changed the `ValueResolveError` type to be a struct, which makes it more opaque and less prone to breakage.

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