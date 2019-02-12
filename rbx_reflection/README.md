# rbx_reflection
Roblox reflection information for working with Instances in external tooling.

This crate is currently a stub. It will hopefully eventually provide:

* Instance types and their members
* Enumerations and their values
* Default values for each property

Much of this data will need to be automatically generated to make this project feasible. Much of it can be pulled from the Roblox JSON API Dump, but information like default values will need more complicated automation.

## Getting default values
The approach that `rbx_reflection` will use to get default values is:

* Generate a built-in plugin that measures default values and sends them over HTTP
* Generate an empty place with HTTP enabled and a marker value in the DataModel
	* The built-in plugin should only activate when this marker is present
* Start up an HTTP server to receive output
* Launch Roblox Studio opening the place file
* Wait for a 'finished' message via HTTP and kill the Roblox Studio process
* Continue with newfound data