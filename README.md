# rbx-tree
rbx-tree is a weakly-typed implemenation of the Roblox DOM. The goal is to have a common format that projects like Rojo can use for handling Instances efficiently.

rbx-tree consists of three crates:

* `rbx_tree`, the actual DOM implementation
* `rbx_xml`, an early serializer and deserializer for the `rbxmx` and `rbxlx` formats
* `rbx_binary`, an early serializer and deserializer for the `rbxm` and `rbxl` formats

rbx-tree is used by [Rojo](https://github.com/LPGhatguy/rojo) for its DOM implementation.

## License
rbx-tree is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.