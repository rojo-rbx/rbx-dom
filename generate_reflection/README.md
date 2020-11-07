# generate_reflection
Generates the reflection database delivered as part of rbx_dom_lua and rbx_reflection_database.

## Requirements
* Windows
* Roblox Studio

## Usage
```bash
generate_reflection [--json <json-path>] [--msgpack <msgpack-path>]
```

## How's it work?
1. Locate Roblox Studio installation
2. Generate API dump via `RobloxStudioBeta -API <output>`
	- The dump is written to a random temporary file which we immediately read back.
	- At this point, we have a lot of information like a list of all instances and their properties, including types!
3. Generate a _derived_ reflection database by merging the JSON API dump with artisanal heuristics defined in [patches](patches).
4. Execute Roblox Studio to get information accessible to Lua
	1. Generate a place file with one copy of every instance in it. It has no properties defined.
	2. Generate a plugin file from [plugin/main.lua](plugin/main.lua).
	3. Install our plugin into Studio's `Plugins` folder
	4. Start an HTTP server to receive messages from the plugin
	5. Start Roblox Studio, opening the generated place
	6. The plugin sends back the current version of studio over HTTP and indicates that Studio has opened successfully.
	7. The operator (you) presses ctrl+s in Studio, saving the generated place.
5. Output the requested reflection databases in msgpack or JSON.