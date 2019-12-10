# generate_rbx_reflection
Generates the source code to `rbx_reflection`. This crate is not intended to be published anywhere and is a tool internal to rbx-dom.

## Requirements
* Windows
* Roblox Studio, installed at the default path (`%LOCALAPPDATA%/Roblox`)

## How's it work?
1. Locate Roblox Studio installation
	- We assume it's under the default Windows installation path right now!
2. Generate API dump via `RobloxStudioBeta -API <output>`
	- The dump is written to a random temporary file which we immediately read back.
	- At this point, we have a lot of information like a list of all instances and their properties, including types!
3. Generate a _derived_ reflection database by merging the JSON API dump with artisanal heuristics defined in [property-patches.toml](property-patches.toml).
4. Execute Roblox Studio to get information accessible to Lua
	1. Generate a built-in plugin file by combining the `plugin` folder with our reflection database and a little wrapper, `src/roblox_plugin_template.lua`.
	2. Install our plugin into Studio's `BuiltInPlugins` folder
	3. Generate a place file with a special marker to activate the plugin
	4. Start an HTTP server to receive messages from the plugin
	5. Start Roblox Studio, opening the generated place
	6. The plugin instantiates every instance from the dump, measures default values, and pushes them to the server.
5. Generate Rust code and put it into `rbx_reflection`
6. Generate Lua code and put it into `rbx_dom_lua`