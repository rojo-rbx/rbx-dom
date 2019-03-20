# generate\_rbx\_reflection
Generates the source code to `rbx_reflection`. This crate is not intended to be published anywhere and is a tool internal to rbx-dom.

## Requirements
* Windows
* Roblox Studio, installed at the default path (`%LOCALAPPDATA%/Roblox`)
* Rojo 0.5.x

## How's it work?
This project is a great example of something called Completely Hacky Arcane Wizardry (CHAW).

1. Locate Roblox Studio installation
	- We assume it's under the default Windows installation path right now. This could be changed to support MacOS in the future.
2. Generate API dump via `RobloxStudioBeta -API <output>`
	- The dump is written to a random temporary file which we immediately read back.
	- At this point, we have a lot of information like a list of all instances and their properties, including types!
3. Execute Roblox Studio to get information accessible to Lua
	1. Generate a built-in plugin file by combinding the `plugin` folder with the JSON API dump and a Lua entrypoint, `src/roblox_plugin_template.lua`.
		- This file is saved to `<studio install>BuiltInPlugins` and doesn't do anything unless the right place is opened
	2. Generate a place file with a special marker to activate the plugin
	3. Start an HTTP server to receive messages from the plugin
	4. Start Roblox Studio, opening the generated place
	5. The plugin instantiates every instance from the dump, measures default values, and pushes them to the server.
4. Generate Rust code and put it into `rbx_reflection`