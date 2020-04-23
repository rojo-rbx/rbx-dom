# rbx_dom_weak
[![rbx_dom_weak on crates.io](https://img.shields.io/crates/v/rbx_dom_weak.svg)](https://crates.io/crates/rbx_dom_weak)
[![rbx_dom_weak docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_dom_weak)

More details about this crate are available on [the rbx-dom GitHub](https://github.com/rojo-rbx/rbx-dom#readme).

Weakly-typed implementation of Roblox's DOM, used for representing instances in external tools.

## Coverage
Because rbx_dom_weak is weakly-typed, it doesn't need to be updated when new instances are added to Roblox. It does, however, have to be updated when new datatypes like `Vector3int16` are added.