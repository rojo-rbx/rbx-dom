# rbx_tree
[![rbx_tree on crates.io](https://img.shields.io/crates/v/rbx_tree.svg)](https://crates.io/crates/rbx_tree)
[![rbx_tree docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_tree)

Weakly-typed implementation of Roblox's DOM, used for representing instances in external tools.

## Coverage
Because rbx_tree is weakly-typed, it doesn't need to be updated when new instances are added to Roblox. It does, however, have to be updated when new datatypes like `Vector3int16` are added.