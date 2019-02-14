# rbx-dom
rbx-dom is a collection of crates to help represent, serialize, and deserialize Roblox DOMs. The goal is to have a common format that projects like [Rojo](https://github.com/LPGhatguy/rojo) can use for handling Instances efficiently.

## [rbx_dom_weak](rbx_dom_weak)
[![rbx_tree on crates.io](https://img.shields.io/crates/v/rbx_tree.svg)](https://crates.io/crates/rbx_tree)
[![rbx_tree docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_tree)

**This crate is in the process of being renamed from `rbx_tree` to `rbx_dom_weak`.**

Weakly-typed Roblox DOM implementation. Defines types for representing instances and properties on them.

## [rbx_xml](rbx_xml)
[![rbx_xml on crates.io](https://img.shields.io/crates/v/rbx_xml.svg)](https://crates.io/crates/rbx_xml)
[![rbx_xml docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_xml)

Serializer and deserializer for for Roblox's XML model and place formats, `rbxmx` and `rbxlx`.

## [rbx_binary](rbx_binary)
[![rbx_binary on crates.io](https://img.shields.io/crates/v/rbx_binary.svg)](https://crates.io/crates/rbx_binary)
[![rbx_binary docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_binary)

Serializer and deserializer for for Roblox's binary model and place formats, `rbxm` and `rbxl`.

## [rbx_reflection](rbx_reflection)
*Not published on crates.io yet*

Roblox reflection information for working with Instances in external tooling.

## Property Type Coverage

| Property Type      | Example Property                | rbx\_tree | rbx\_xml | rbx\_binary |
| ------------------ | ------------------------------- |:---------:|:--------:|:-----------:|
| Axes               | `ArcHandles.Axes`               | ❌ | ❌ | ❌ |
| BinaryString       | `Terrain.MaterialColors`        | ✔ | ✔ | ❌ |
| Bool               | `Part.Anchored`                 | ✔ | ✔ | ✔ |
| BrickColor         | `Part.BrickColor`               | ❌ | ❌ | ❌ |
| CFrame             | `Camera.CFrame`                 | ✔ | ✔ | ❌ |
| Color3             | `Lighting.Ambient`              | ✔ | ✔ | ❌ |
| Color3uint8        | `N/A`                           | ✔ | ✔ | ❌ |
| ColorSequence      | `Beam.Color`                    | ❌ | ❌ | ❌ |
| Content            | `Decal.Texture`                 | ❌ | ❌ | ❌ |
| Enum               | `Part.Shape`                    | ✔ | ✔ | ❌ |
| Faces              | `BasePart.ResizableFaces`       | ❌ | ❌ | ❌ |
| Float32            | `Players.RespawnTime`           | ✔ | ✔ | ❌ |
| Float64            | `PlaybackLoudness`              | ❌ | ❌ | ❌ |
| Int32              | `Frame.ZIndex`                  | ✔ | ✔ | ❌ |
| Int64              | `Player.UserId`                 | ❌ | ❌ | ❌ |
| NumberRange        | `ParticleEmitter.Lifetime`      | ❌ | ❌ | ❌ |
| NumberSequence     | `Beam.Transparency`             | ❌ | ❌ | ❌ |
| PhysicalProperties | `Part.CustomPhysicalProperties` | ➖ | ➖ | ❌ |
| ProtectedString    | `ModuleScript.Source`           | ✔¹ | ✔¹ | ❌ |
| QDir               | `Studio.Auto-Save Path`         | ❌ | ❌ | ❌ |
| QFont              | `Studio.Font`                   | ❌ | ❌ | ❌ |
| Ray                | `RayValue.Value`                | ❌ | ❌ | ❌ |
| Rect2D             | `ImageButton.SliceCenter`       | ❌ | ❌ | ❌ |
| Ref                | `Model.PrimaryPart`             | ✔ | ✔ | ❌ |
| Region3int16       | `Terrain.MaxExtents`            | ❌ | ❌ | ❌ |
| String             | `Instance.Name`                 | ✔ | ✔ | ✔ |
| UDim               | `UIListLayout.Padding`          | ❌ | ❌ | ❌ |
| UDim2              | `Frame.Size`                    | ❌ | ❌ | ❌ |
| Vector2            | `ImageLabel.ImageRectSize`      | ✔ | ✔ | ❌ |
| Vector2int16       | `N/A`                           | ✔ | ✔ | ❌ |
| Vector3            | `Part.Size`                     | ✔ | ✔ | ❌ |
| Vector3int16       | `N/A`                           | ✔ | ✔ | ❌ |

✔ Implemented | ❌ Unimplemented | ➖ Partially Implemented

1. ProtectedString is deserialized as String, which is technically lossy but does not change semantics in practice

## License
rbx-dom is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.