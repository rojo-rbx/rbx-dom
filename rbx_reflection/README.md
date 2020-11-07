# rbx_reflection
[![rbx_reflection on crates.io](https://img.shields.io/crates/v/rbx_reflection.svg)](https://crates.io/crates/rbx_reflection)
[![rbx_reflection docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_reflection)

More details about this crate are available on [the rbx-dom GitHub](https://github.com/rojo-rbx/rbx-dom#readme).

Contains the types needed to describe a Roblox reflection database.

rbx_reflection can describe:

* All Instance types and their members
* All Enums and their values
* Default values for each property
* The Roblox client version this information came from

Starting in rbx_reflection 4.0, there is no longer a bundled database in this crate. Look at the [rbx_reflection_database crate](http://crates.io/crates/rbx_reflection_database) for a generated database that is easy to depend on.