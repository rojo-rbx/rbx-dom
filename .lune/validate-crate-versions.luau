--!strict
--#selene: allow(incorrect_standard_library_use)

local IGNORE_CRATE_LIST = {
	"rbx_util",
	"rbx_reflector",
}

local fs = require("@lune/fs")
local serde = require("@lune/serde")
local stdio = require("@lune/stdio")
local process = require("@lune/process")

local SemVer = require("semver")

type WorkspaceCargo = {
	workspace: {
		members: { string },
	},
}

type CrateCargo = {
	package: {
		name: string,
		version: string,
	},
	dependencies: { [string]: Dependency },
	["dev-dependencies"]: { [string]: Dependency }?,
}

type Dependency = string | { version: string?, path: string?, features: { string }, optional: boolean? }

local function warn(...)
	stdio.write(`[{stdio.color("yellow")}WARN{stdio.color("reset")}] `)
	print(...)
end

local function processDependencies(dependency_list: { [string]: Dependency }, output: { [string]: Dependency })
	for name, dependency in dependency_list do
		if typeof(dependency) == "string" then
			continue
		end
		if dependency.path then
			output[name] = dependency
		end
	end
end

local workspace: WorkspaceCargo = serde.decode("toml", fs.readFile("Cargo.toml"))

local crate_info = {}

for _, crate_name in workspace.workspace.members do
	if table.find(IGNORE_CRATE_LIST, crate_name) then
		continue
	end
	local cargo: CrateCargo = serde.decode("toml", fs.readFile(`{crate_name}/Cargo.toml`))
	local dependencies = {}
	local dev_dependencies = {}
	processDependencies(cargo.dependencies, dependencies)
	if cargo.package["dev-dependencies"] then
		processDependencies(cargo["dev-dependencies"] :: any, dev_dependencies)
	end
	crate_info[crate_name] = {
		version = SemVer.parse(cargo.package.version),
		dependencies = dependencies,
		dev_dependencies = dev_dependencies,
	}
end

table.freeze(crate_info)

local all_ok = true

for crate_name, cargo in crate_info do
	for name, dependency in cargo.dependencies do
		if typeof(dependency) == "string" then
			error("invariant: string dependency made it into path list")
		end
		if not crate_info[name] then
			warn(`Dependency {name} of crate {crate_name} has a path but is not local to this workspace.`)
			all_ok = false
			continue
		end
		if not dependency.version then
			warn(`Dependency {name} of crate {crate_name} has a path but no version specified. Please fix this.`)
			all_ok = false
			continue
		end
		local dependency_version = SemVer.parse(dependency.version :: string)
		local cmp = SemVer.compare(crate_info[name].version, dependency_version)
		if cmp == 0 then
			continue
		else
			all_ok = false
			warn(
				`Dependency {name} of crate {crate_name} has a version mismatch. Current version: {dependency_version}. Should be: {crate_info[name].version}`
			)
		end
	end
end

if all_ok then
	process.exit(0)
else
	process.exit(1)
end
