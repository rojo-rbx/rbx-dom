# rbx_types
[![rbx_types on crates.io](https://img.shields.io/crates/v/rbx_types.svg)](https://crates.io/crates/rbx_types)
[![rbx_types docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_types)

More details about this crate are available on [the rbx-dom GitHub](https://github.com/rojo-rbx/rbx-dom#readme).

rbx_types is a crate containing value types for interacting with Roblox.

rbx_types contains value types, which are generally evergreen and are added infrequently. These kinds of type are in scope for rbx_types:

- `Vector3`, Roblox's 3D vector type
- `NumberSequence`, a series of numbers on a timeline
- `BinaryString`, an opaque buffer of binary data
- More specialized data types representing specific `BinaryString` blobs

rbx_types will _not_ contain instance types. New instances are added to the platform all the time and have more complicated semantics.
