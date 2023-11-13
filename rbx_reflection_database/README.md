# rbx_reflection_database
[![rbx_reflection_database on crates.io](https://img.shields.io/crates/v/rbx_reflection_database.svg)](https://crates.io/crates/rbx_reflection_database)
[![rbx_reflection_database docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_reflection_database)

More details about this crate are available on [the rbx-dom GitHub](https://github.com/rojo-rbx/rbx-dom#readme).

Contains an API to get a Roblox reflection database using the types from [`rbx_reflection`](https://crates.io/crates/rbx_reflection). This crate embeds a database for this purpose, but also provides an API for dependents to get a reflection database from a consistent location.

The general way this crate should be used is via `get`. This method will search for a locally stored reflection database and return it if it's found. If it isn't, it will instead return the bundled one. The details for where it searches are below.

Additionally, this crate exposes `get_local` and `get_bundled` for easily only loading the locally stored database or only the bundled one respectively.

## Local Details

This crate will load a reflection database from the file system if one exists in the default location. This location varies upon the OS and is specified here:

| OS      | Location                                                            |
|:--------|:--------------------------------------------------------------------|
| Windows | `%localappdata%/.rbxreflection/database.msgpack`                    |
| MacOS   | `$HOME/Library/Application Support/.rbxreflection/database.msgpack` |
| Linux   | `$HOME/.rbxreflection/database.msgpack`                             |

Additionally, a location override may be specified via the `RBX_DATABASE` environmental variable.

Both the default `database.msgpack` files and any files pointed to by `RBX_DATABASE` must be valid MessagePack serializations of a [`ReflectionDatabase`][ReflectionDatabase] if they're present.

[ReflectionDatabase]: https://docs.rs/rbx_reflection/latest/rbx_reflection/struct.ReflectionDatabase.html