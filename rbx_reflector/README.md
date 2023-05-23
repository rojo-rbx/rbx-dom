# rbx_reflector
Generates the reflection database delivered as part of rbx_dom_lua and rbx_reflection_database.

## Requirements
* Roblox Studio

## Usage
```bash
rbx_reflector generate [--patches <PATCHES>] [OUTPUTS]
```

## Example
```bash
rbx_reflector generate --patches patches rbx_reflection_database/database.msgpack rbx_dom_lua/src/database.json
```

## How's it work?
1. Locate Roblox Studio installation
2. Generate API dump via `RobloxStudioBeta -FullAPI <output>`
	- The dump is written to a temporary file which we immediately read back.
	- At this point, we have a lot of information like a list of all instances and their properties, including types!
3. Generate a _derived_ reflection database by merging the JSON API dump with artisanal heuristics defined in [patches](patches).
4. Execute Roblox Studio to get information accessible to Lua
	1. Generate a place file with one copy of every instance in it. It has no properties defined.
	2. Generate a plugin file from [plugin.lua](plugin.lua).
	3. Install our plugin into Studio's `Plugins` folder
	4. Start an HTTP server to receive messages from the plugin
	5. Start Roblox Studio, opening the generated place
	6. The plugin indicates over HTTP that Studio has opened successfully.
    7. The place is automatically saved on Windows or requires you to press crtl+s to save.
5. Output the requested reflection databases in msgpack or JSON.