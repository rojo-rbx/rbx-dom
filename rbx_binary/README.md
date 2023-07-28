# rbx_binary
[![rbx_binary on crates.io](https://img.shields.io/crates/v/rbx_binary.svg)](https://crates.io/crates/rbx_binary)
[![rbx_binary docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_binary)

More details about this crate are available on [the rbx-dom GitHub](https://github.com/rojo-rbx/rbx-dom#readme).

Implementation of Roblox's binary model formats, rbxm and rbxl, for the rbx-dom ecosystem.

## Coverage

rbx_binary supports all property types from [`rbx_types`](https://crates.io/crates/rbx_types). It currently embeds a reflection database which contains a list of classes, properties, their types, and other information. In the event that this database is outdated, some issues may come up.

These are mostly harmless, but they may include:
	- Properties being serialized under the wrong name
	- Properties having the incorrect default

Additionally, data types newer than the current release of rbx_binary cannot be (de)serialized.

In all of these cases, an update is needed for this crate.