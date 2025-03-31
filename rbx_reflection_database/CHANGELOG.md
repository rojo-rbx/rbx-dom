# rbx\_reflection_database Changelog

## Unreleased Changes

## 1.0.1+roblox-666 (2025-03-31)
* Corrected the serialization of `MeshPart.TextureID` to migrate to `MeshPart.TextureContent` ([#505])

[#505]: https://github.com/rojo-rbx/rbx-dom/pull/505

## 1.0.0+roblox-666 (2025-03-28)
* Update to Roblox version 666.
* `Model.WorldPivotData`'s default value is now `null`. ([#450])
* `SharedString` properties now have default values. ([#458])
* `Instance.DefinesCapabilities` is now an alias for `Instance.Sandboxed` ([#459])
* `BasePart.MaterialVariant` is now an alias for `BasePart.MaterialVariantSerialized` ([#474])
* Updated other rbx_dom dependencies, which results in a breaking change to `Variant`

[#458]: https://github.com/rojo-rbx/rbx-dom/pull/458
[#450]: https://github.com/rojo-rbx/rbx-dom/pull/450
[#459]: https://github.com/rojo-rbx/rbx-dom/pull/459
[#474]: https://github.com/rojo-rbx/rbx-dom/pull/474

## 0.2.12+roblox-638 (2024-08-22)
* Update to Roblox version 638.
* `Instance.UniqueId`, `Instance.HistoryId`, and `LuaSourceContainer.ScriptGuid` are marked as `Serializes` again ([#437])

[#437]: https://github.com/rojo-rbx/rbx-dom/pull/437

# 0.2.11+roblox-634 (2024-07-23)
* Updated to Roblox version 634

## 0.2.10+roblox-607 (2024-01-16)
* Updated to Roblox version 607

## 0.2.9+roblox-596 (2023-10-03)
## 0.2.8+roblox-296 (incorrect metadata)
* Updated to Roblox version 596.
* Added properties of type `SecurityCapabilities`. ([#359])

[#359]: https://github.com/rojo-rbx/rbx-dom/pull/359

## 0.2.7+roblox-588
* Updated to Roblox version 588.
* `Instance.UniqueId`, `Instance.HistoryId`, and `LuaSourceContainer.ScriptGuid` are now marked as `DoesNotSerialize` ([#327])

[#327]: https://github.com/rojo-rbx/rbx-dom/pull/327

## 0.2.6+roblox-572
* Updated to Roblox version 572.

## 0.2.5+roblox-530
* Updated to Roblox version 530.
* `Instance.Attributes` is now the canonical version of `Instance.AttributesSerialize` and is marked as scriptable.

## 0.2.4+roblox-504
* Updated to Roblox version 504.
* Fixed `WeldConstraint` `Part0` and `Part1` aliases being incomplete. `Part0` and `Part1` properties should now serialize correctly.

## 0.2.3+roblox-503
* Updated to Roblox version 503.

## 0.2.2+roblox-498
* Updated to rbx_reflection 4.2.0
* Updated to Roblox version 498.
* Added aliases `Part0` and `Part1` for `WeldConstraint.Part0Internal` and `WeldConstraint.Part1Internal`.
* Changed type of `Instance.Tags` from `BinaryString` to `Tags`. ([#199])

[#199]: https://github.com/rojo-rbx/rbx-dom/pull/199

## 0.2.1+roblox-484 (2021-07-02)
* Updated to rbx_types 1.1.
* Updated to Roblox version 484.

## 0.2.0+roblox-478 (2021-06-26) (yanked)
* Updated to rbx_reflection 4.0 stable.

## 0.1.1+roblox-478 (2021-05-14)
* Updated to Roblox version 478.
* Fixed scriptability of properties ([#177])
* Fixed PackageId serialization ([#175])

[#177]: https://github.com/rojo-rbx/rbx-dom/pull/177
[#175]: https://github.com/rojo-rbx/rbx-dom/pull/175

## 0.1.0+roblox-465 (2021-02-16)
* Initial release, based on Roblox version 465.
