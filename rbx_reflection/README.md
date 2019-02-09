# rbx_reflection
Roblox reflection information for working with Instances in external tooling.

This crate is currently a stub. It will hopefully eventually provide:

* Instance types and their members
* Enumerations and their values
* Default values for each property

Much of this data will need to be automatically generated to make this project feasible. Much of it can be pulled from the Roblox JSON API Dump, but information like default values will need more complicated automation.

## Getting default values
The approach that `rbx_reflection` will use to get default values is:

* Generate a place file with Rojo that consumes the API dump and measures default properties
* Start up an HTTP server to receive output
* Launch Roblox Studio opening the place file, prompting the user to press f5
* Dump data via HTTP back to the server, output onto the filesystem