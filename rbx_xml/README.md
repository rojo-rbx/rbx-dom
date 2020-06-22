# rbx_xml
[![rbx_xml on crates.io](https://img.shields.io/crates/v/rbx_xml.svg)](https://crates.io/crates/rbx_xml)
[![rbx_xml docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_xml)

More details about this crate are available on [the rbx-dom GitHub](https://github.com/Roblox/rbx-dom#readme).

Implementation of Roblox's XML model formats, rbxmx and rbxlx for the rbx-dom ecosystem.

## Coverage
rbx_xml aims to support all property types from rbx_dom_weak.

Some properties serialize with different names in XML than the names exposed via Roblox's API dump or via the Lua API. In those cases, rbx_xml keeps a mapping that needs to be kept up to date. These cases are pretty uncommon, so that table is small.