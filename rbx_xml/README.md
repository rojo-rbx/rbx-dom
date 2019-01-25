# rbx_xml
[![rbx_xml on crates.io](https://img.shields.io/crates/v/rbx_xml.svg)](https://crates.io/crates/rbx_xml)
[![rbx_xml docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_xml)

Implementation of Roblox's XML model formats, rbxmx and rbxlx, for rbx-tree.

## Coverage
rbx\_xml aims to support all property types from rbx\_tree.

Some properties serialize with different names in XML than the names exposed via Roblox's API dump or via the Lua API. In those cases, rbx_xml keeps a mapping that needs to be kept up to date. These cases are pretty uncommon, so that table is small.

Data type coverage:

* [ ] Content
* [ ] PhysicalProperties (currently a stub)
* [ ] Ref
* [x] BinaryString
* [x] Bool
* [x] CFrame
* [x] Color3
* [x] Color3uint8
* [x] Enum
* [x] Float32
* [x] Int32
* [x] String
* [x] Vector2
* [x] Vector2int16
* [x] Vector3
* [x] Vector3int16