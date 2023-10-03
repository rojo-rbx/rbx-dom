<div align="center">
	<img width="400" src="rbx-dom-logo.png" />
</div>

<h1 align="center">rbx-dom</h1>
<div align="center">
	<a href="https://github.com/rojo-rbx/rbx-dom/actions">
		<img title="GitHub Actions" src="https://github.com/rojo-rbx/rbx-dom/workflows/CI/badge.svg" />
	</a>
</div>

rbx-dom is a collection of cross-platform libraries that enables any software to interact with Roblox instances.

Documentation about the project is hosted at [dom.rojo.space](https://dom.rojo.space).

At this moment, we do not specify a MSRV for any project in rbx-dom. If you need to build one of these libraries with an outdated version of Rust, please open an issue explaining what blockers there are to you updating Cargo, what version you would need us to support, and why.

## [rbx_dom_weak](rbx_dom_weak)
[![rbx_dom_weak on crates.io](https://img.shields.io/crates/v/rbx_dom_weak.svg)](https://crates.io/crates/rbx_dom_weak)
[![rbx_dom_weak docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_dom_weak)

Weakly-typed Roblox DOM implementation. Defines types for representing instances and properties on them.

## [rbx_types](rbx_types)
[![rbx_types on crates.io](https://img.shields.io/crates/v/rbx_types.svg)](https://crates.io/crates/rbx_types)
[![rbx_types docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_types)

Contains Roblox's value types like `Vector3` and `NumberSequence`. Used by crates like rbx_dom_weak and a future rbx_dom_strong crate to let them share types and conversions.

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

Roblox reflection types for working with Instances in external tooling.

## [rbx_reflection_database](rbx_reflection_database)
[![rbx_reflection_database on crates.io](https://img.shields.io/crates/v/rbx_reflection_database.svg)](https://crates.io/crates/rbx_reflection_database)
[![rbx_reflection_database docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_reflection_database)

Bundled reflection database using types from rbx_reflection. Intended for users migrating from rbx_reflection 4.x and users who need reflection information statically.

## [rbx_reflector](rbx_reflector)

Command line utility to generate a reflection database for rbx_dom_lua and rbx_reflection_database.

## [rbx_util](rbx_util)

Command line utility to convert and debug Roblox model files.

## [rbx_dom_lua](rbx_dom_lua)

Roblox Lua implementation of DOM APIs, allowing Instance reflection from inside Roblox. Uses a data format that's compatible with rbx_dom_weak to facilitate communication with applications outside Roblox about instances.

## Property Type Coverage

| Property Type           | Example Property                | rbx_types | rbx_dom_lua | rbx_xml | rbx_binary
|:------------------------|:--------------------------------|:--:|:--:|:--:|:--:|
| Axes                    | `ArcHandles.Axes`               | ✔ | ✔ | ✔ | ✔ |
| BinaryString            | `Terrain.MaterialColors`        | ✔ | ➖ | ✔ | ✔ |
| Bool                    | `Part.Anchored`                 | ✔ | ✔ | ✔ | ✔ |
| BrickColor              | `Part.BrickColor`               | ✔ | ✔ | ✔ | ✔ |
| Bytecode                | N/A                             | ❌ | ⛔ | ❌ | ❌ |
| CFrame                  | `Camera.CFrame`                 | ✔ | ✔ | ✔ | ✔ |
| Color3                  | `Lighting.Ambient`              | ✔ | ✔ | ✔ | ✔ |
| Color3uint8             | `Part.BrickColor`               | ✔ | ✔ | ✔ | ✔ |
| ColorSequence           | `Beam.Color`                    | ✔ | ✔ | ✔ | ✔ |
| Content                 | `Decal.Texture`                 | ✔ | ✔ | ✔ | ✔ |
| Enum                    | `Part.Shape`                    | ✔ | ✔ | ✔ | ✔ |
| Faces                   | `Handles.Faces`                 | ✔ | ✔ | ✔ | ✔ |
| Float32                 | `Players.RespawnTime`           | ✔ | ✔ | ✔ | ✔ |
| Float64                 | `Sound.PlaybackLoudness`        | ✔ | ✔ | ✔ | ✔ |
| Font                    | `TextLabel.Font`                | ✔ | ✔ | ✔ | ✔ |
| Int32                   | `Frame.ZIndex`                  | ✔ | ✔ | ✔ | ✔ |
| Int64                   | `Player.UserId`                 | ✔ | ✔ | ✔ | ✔ |
| NumberRange             | `ParticleEmitter.Lifetime`      | ✔ | ✔ | ✔ | ✔ |
| NumberSequence          | `Beam.Transparency`             | ✔ | ✔ | ✔ | ✔ |
| OptionalCoordinateFrame | `Model.WorldPivotData`          | ✔ | ❌ | ✔ | ✔ |
| PhysicalProperties      | `Part.CustomPhysicalProperties` | ✔ | ✔ | ✔ | ✔ |
| ProtectedString         | `ModuleScript.Source`           | ✔ | ✔ | ✔ | ✔ |
| Ray                     | `RayValue.Value`                | ✔ | ✔ | ✔ | ✔ |
| Rect                    | `ImageButton.SliceCenter`       | ✔ | ✔ | ✔ | ✔ |
| Ref                     | `Model.PrimaryPart`             | ✔ | ✔ | ✔ | ✔ |
| Region3                 | N/A                             | ✔ | ✔ | ❌ | ❌ |
| Region3int16            | `Terrain.MaxExtents`            | ✔ | ✔ | ❌ | ❌ |
| SecurityCapabilities    | `Folder.SecurityCapabilities`   | ✔ | ❌ | ✔ | ✔ |
| SharedString            | N/A                             | ✔ | ✔ | ✔ | ✔ |
| String                  | `Instance.Name`                 | ✔ | ✔ | ✔ | ✔ |
| UDim                    | `UIListLayout.Padding`          | ✔ | ✔ | ✔ | ✔ |
| UDim2                   | `Frame.Size`                    | ✔ | ✔ | ✔ | ✔ |
| UniqueId                | `Instance.UniqueId`             | ✔ | ❌ | ✔ | ✔ |
| Vector2                 | `ImageLabel.ImageRectSize`      | ✔ | ✔ | ✔ | ✔ |
| Vector2int16            | N/A                             | ✔ | ✔ | ✔ | ❌ |
| Vector3                 | `Part.Size`                     | ✔ | ✔ | ✔ | ✔ |
| Vector3int16            | `TerrainRegion.ExtentsMax`      | ✔ | ✔ | ✔ | ✔ |
| QDir                    | `Studio.Auto-Save Path`         | ⛔ | ⛔ | ⛔ | ⛔ |
| QFont                   | `Studio.Font`                   | ⛔ | ⛔ | ⛔ | ⛔ |

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

## For Maintainers

Cutting new releases is not currently as optimized as it should be. While we work on improving it, packages need to be published in a specific order to make sense. The order that currently works well is:

1. `rbx_types`
2. `rbx_dom_weak` and `rbx_reflection`
3. `rbx_reflection_database`
4. `rbx_binary` and `rbx_xml`

The process for publishing these is:

1. Decide a new version number, following [SemVer](semver.org/)
2. Update changelog to list new release under its own heading
3. Adjust versions of local dependencies to be the new release (this is why releases must happen in a specific order)
4. Increment version in `Cargo.toml`
5. Add a git tag in the format `library_name-vMAJOR.MINOR.PATCH` at the commit that incremented the Cargo version
6. Publish to Cargo

## License
rbx-dom is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.
