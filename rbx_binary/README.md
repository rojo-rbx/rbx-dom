# rbx_binary
[![rbx_binary on crates.io](https://img.shields.io/crates/v/rbx_binary.svg)](https://crates.io/crates/rbx_binary)
[![rbx_binary docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_binary)

Implementation of Roblox's binary model formats, rbxm and rbxl, for rbx-tree.

## Coverage
rbx\_xml aims to support all property types from rbx\_tree, but it currently lags behind rbx\_xml due to implementation complexity.

Data type coverage:

* [x] Bool
* [x] String
* [ ] BinaryString
* [ ] CFrame
* [ ] Color3
* [ ] Color3uint8
* [ ] Content
* [ ] Enum
* [ ] Float32
* [ ] Int32
* [ ] PhysicalProperties (currently a stub)
* [ ] Ref
* [ ] Vector2
* [ ] Vector2int16
* [ ] Vector3
* [ ] Vector3int16

## Format
For a specification of what the format looks like, see [FORMAT.md](FORMAT.md).