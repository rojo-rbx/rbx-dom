# rbx-tree
rbx-tree is a weakly-typed implemenation of the Roblox DOM. The goal is to have a common format that projects like [Rojo](https://github.com/LPGhatguy/rojo) can use for handling Instances efficiently.

## [rbx_tree](rbx_tree)
[![rbx_tree on crates.io](https://img.shields.io/crates/v/rbx_tree.svg)](https://crates.io/crates/rbx_tree)
[![rbx_tree docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_tree)

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
| BinaryString       | `Terrain.MaterialColors`        | ✔ | ✔ | ❌ |
| Bool               | `Part.Anchored`                 | ✔ | ✔ | ✔ |
| CFrame             | `Camera.CFrame`                 | ✔ | ✔ | ❌ |
| Color3             | `Lighting.Ambient`              | ✔ | ✔ | ❌ |
| Color3uint8        | `N/A`                           | ✔ | ✔ | ❌ |
| Content            | `Decal.Texture`                 | ❌ | ❌ | ❌ |
| Enum               | `Part.Shape`                    | ✔ | ✔ | ❌ |
| Float32            | `Players.RespawnTime`           | ✔ | ✔ | ❌ |
| Int32              | `Frame.ZIndex`                  | ✔ | ✔ | ❌ |
| PhysicalProperties | `Part.CustomPhysicalProperties` | ➖ | ➖ | ❌ |
| Rect2D             | `ImageButton.SliceCenter`       | ❌ | ❌ | ❌ |
| Ref                | `Model.PrimaryPart`             | ❌ | ❌ | ❌ |
| String             | `Instance.Name`                 | ✔ | ✔ | ✔ |
| UDim2              | `Frame.Size`                    | ❌ | ❌ | ❌ |
| Vector2            | `ImageLabel.ImageRectSize`      | ✔ | ✔ | ❌ |
| Vector2int16       | `N/A`                           | ✔ | ✔ | ❌ |
| Vector3            | `Part.Size`                     | ✔ | ✔ | ❌ |
| Vector3int16       | `N/A`                           | ✔ | ✔ | ❌ |
| ProtectedString    | `ModuleScript.Source`           | ✔¹ | ✔¹ | ❌ |

✔ Implemented | ❌ Unimplemented | ➖ Partially Implemented

1. ProtectedString is deserialized as String, which is technically lossy but does not change semantics in practice

## License
rbx-tree is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.