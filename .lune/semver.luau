--!strict
local SemVer = {}
SemVer.__index = SemVer

export type SemVer = {
	major: number,
	minor: number?,
	patch: number?,
}

local function compare(a: number, b: number): number
	if a > b then
		return 1
	elseif a < b then
		return -1
	end

	return 0
end

function SemVer.__tostring(self: SemVer): string
	return string.format("%i.%i.%i", self.major, self.minor or 0, self.patch or 0)
end

--[[
	Constructs a new SemVer from the provided parts.
]]
function SemVer.new(major: number, minor: number?, patch: number?): SemVer
	return (setmetatable({
		major = major,
		minor = minor,
		patch = patch,
	}, SemVer) :: any) :: SemVer
end

--[[
    Parses `version` into a SemVer.
]]
function SemVer.parse(version: string)
	local major, minor, patch, _ = version:match("^(%d+)%.(%d+)%.(%d+)(.*)$")
	local realVersion = {
		major = tonumber(major),
		minor = tonumber(minor),
		patch = tonumber(patch),
	}
	if realVersion.major == nil then
		error(`could not parse major version from '{version}'`, 2)
	end
	if minor and realVersion.minor == nil then
		error(`could not parse minor version from '{version}'`, 2)
	end
	if patch and realVersion.patch == nil then
		error(`could not parse patch version from '{version}'`, 2)
	end

	return (setmetatable(realVersion, SemVer) :: any) :: SemVer
end

--[[
	Compares two SemVers and returns their status compared to one another.
	
	If `1` is returned, a is 'newer' than b.
	If `-1` is returned, a is 'older' than b.
	If `0` is returned, they are identical.
]]
function SemVer.compare(a: SemVer, b: SemVer)
	local major = compare(a.major, b.major)
	local minor = compare(a.minor or 0, b.minor or 0)

	if major ~= 0 then
		return major
	end

	if minor ~= 0 then
		return minor
	end

	return compare(a.patch or 0, b.patch or 0)
end

return SemVer
