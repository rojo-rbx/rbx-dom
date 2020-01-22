# rbx-dom
[![GitHub Actions](https://github.com/rojo-rbx/rbx-dom/workflows/CI/badge.svg)](https://github.com/rojo-rbx/rbx-dom/actions)

rbx-dom is a collection of crates to help represent, serialize, and deserialize Roblox DOMs. The goal of rbx-dom is to have a common representation that projects like [Rojo](https://github.com/rojo-rbx/rojo) can use for handling Instances efficiently.

## [rbx_dom_weak](rbx_dom_weak)
[![rbx_dom_weak on crates.io](https://img.shields.io/crates/v/rbx_dom_weak.svg)](https://crates.io/crates/rbx_dom_weak)
[![rbx_dom_weak docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_dom_weak)

Weakly-typed Roblox DOM implementation. Defines types for representing instances and properties on them.

## [rbx_dom_lua](rbx_dom_lua)

Roblox Lua implementation of DOM APIs, allowing Instance reflection from inside Roblox. Uses a data format that's compatible with rbx_dom_weak to facilitate communication with applications outside Roblox about instances.

## [rbx_xml](rbx_xml)
[![rbx_xml on crates.io](https://img.shields.io/crates/v/rbx_xml.svg)](https://crates.io/crates/rbx_xml)
[![rbx_xml docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_xml)

Serializer and deserializer for for Roblox's XML model and place formats, `rbxmx` and `rbxlx`.

## [rbx_binary](rbx_binary)
[![rbx_binary on crates.io](https://img.shields.io/crates/v/rbx_binary.svg)](https://crates.io/crates/rbx_binary)
[![rbx_binary docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_binary)

Serializer and deserializer for for Roblox's binary model and place formats, `rbxm` and `rbxl`.

## [rbx_reflection](rbx_reflection)
[![rbx_reflection on crates.io](https://img.shields.io/crates/v/rbx_reflection.svg)](https://crates.io/crates/rbx_reflection)
[![rbx_reflection docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_reflection)

Roblox reflection information for working with Instances in external tooling.

## Property Type Coverage

| Property Type      | Example Property                | rbx_dom_weak | rbx_dom_lua | rbx_xml | rbx_binary
|:------------------ |:------------------------------- |:--:|:--:|:--:|:--:|
| Axes               | `ArcHandles.Axes`               | ❌ | ❌ | ❌ | ❌ |
| BinaryString       | `Terrain.MaterialColors`        | ✔ | ➖ | ✔ | ❌ |
| Bool               | `Part.Anchored`                 | ✔ | ✔ | ✔ | ✔ |
| BrickColor         | `Part.BrickColor`               | ✔ | ✔ | ✔ | ❌ |
| CFrame             | `Camera.CFrame`                 | ✔ | ✔ | ✔ | ❌ |
| Color3             | `Lighting.Ambient`              | ✔ | ✔ | ✔ | ❌ |
| Color3uint8        | `N/A`                           | ✔ | ✔ | ✔ | ❌ |
| ColorSequence      | `Beam.Color`                    | ✔ | ✔ | ✔ | ❌ |
| Content            | `Decal.Texture`                 | ✔ | ✔ | ✔ | ✔ |
| Enum               | `Part.Shape`                    | ✔ | ✔ | ✔ | ❌ |
| Faces              | `BasePart.ResizableFaces`       | ❌ | ❌ | ❌ | ❌ |
| Float32            | `Players.RespawnTime`           | ✔ | ✔ | ✔ | ❌ |
| Float64            | `Sound.PlaybackLoudness`        | ✔ | ✔ | ✔ | ❌ |
| Int32              | `Frame.ZIndex`                  | ✔ | ✔ | ✔ | ❌ |
| Int64              | `Player.UserId`                 | ✔ | ✔ | ✔ | ❌ |
| NumberRange        | `ParticleEmitter.Lifetime`      | ✔ | ✔ | ✔ | ❌ |
| NumberSequence     | `Beam.Transparency`             | ✔ | ✔ | ✔ | ❌ |
| PhysicalProperties | `Part.CustomPhysicalProperties` | ✔ | ✔ | ✔ | ❌ |
| ProtectedString    | `ModuleScript.Source`           | ✔ | ✔ | ✔ | ✔ |
| Ray                | `RayValue.Value`                | ✔ | ❌ | ✔ | ❌ |
| Rect               | `ImageButton.SliceCenter`       | ✔ | ✔ | ✔ | ❌ |
| Ref                | `Model.PrimaryPart`             | ✔ | ✔ | ✔ | ❌ |
| Region3            | `N/A`                           | ❌ | ✔ | ❌ | ❌ |
| Region3int16       | `Terrain.MaxExtents`            | ❌ | ✔ | ❌ | ❌ |
| SharedString       | `N/A`                           | ✔ | ✔ | ✔ | ❌ |
| String             | `Instance.Name`                 | ✔ | ✔ | ✔ | ✔ |
| UDim               | `UIListLayout.Padding`          | ✔ | ✔ | ✔ | ❌ |
| UDim2              | `Frame.Size`                    | ✔ | ✔ | ✔ | ❌ |
| Vector2            | `ImageLabel.ImageRectSize`      | ✔ | ✔ | ✔ | ❌ |
| Vector2int16       | `N/A`                           | ✔ | ✔ | ✔ | ❌ |
| Vector3            | `Part.Size`                     | ✔ | ✔ | ✔ | ❌ |
| Vector3int16       | `N/A`                           | ✔ | ✔ | ✔ | ❌ |
| QDir               | `Studio.Auto-Save Path`         | ⛔ | ⛔ | ⛔ | ⛔ |
| QFont              | `Studio.Font`                   | ⛔ | ⛔ | ⛔ | ⛔ |

✔ Implemented | ❌ Unimplemented | ➖ Partially Implemented | ⛔ Never

## Outcome
This project has unveiled a handful of interesting bugs and quirks in Roblox!

- `GuiMain.DisplayOrder` is uninitialized, so its default value isn't stable
- `MaxPlayersInternal` and `PreferredPlayersInternal` on `Players` are scriptable and accessible by the command bar
- Instantiating a `NetworkClient` will turn your edit session into a game client and stop you from sending HTTP requests
- `ContentProvider.RequestQueueSize` is mistakenly marked as serializable
- Trying to invoke `game:GetService("Studio")` causes a unique error: `singleton Studio already exists`
- `Color3` properties not serialized as `Color3uint8` would have their colors mistakenly clamped in the XML place format. This was bad for properties on `Lighting`.
- `ColorSequence`'s XML serialization contains an extra value per keypoint that was intended to be used as an envelope value, but was never implemented.

## Minimum Rust Version
This project tries (via CI) to support Rust 1.36.0 and newer. Rust 1.36.0 was released July 4, 2019.

## License
rbx-dom is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.