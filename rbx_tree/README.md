# rbx_tree
[![rbx_tree on crates.io](https://img.shields.io/crates/v/rbx_tree.svg)](https://crates.io/crates/rbx_tree)
[![rbx_tree docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_tree)

Weakly-typed implementation of Roblox's DOM, used for representing instances in external tools.

## Coverage
Because rbx_tree is weakly-typed, it doesn't need to be updated when new instances are added to Roblox. It does, however, have to be updated when new datatypes like `Vector3int16` are added.

Data type coverage:

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
* [ ] Content
* [ ] PhysicalProperties (currently stubbed)
* [ ] Ref