local HttpService = game:GetService("HttpService")

local VERBOSE = false
local ERROR_AT_END = false

local EncodedValue = require(script.Parent.EncodedValue)

local ReflectionClasses = require(script.Parent.ReflectionClasses)

local function verbosePrint(...)
	if VERBOSE then
		print(...)
	end
end

--[[
	Little function for use with pcall to avoid making so many closures.
]]
local function get(parent, key)
	return parent[key]
end

local function set(parent, key, value)
	parent[key] = value
end

local function getClientVersion()
	local version = string.split(version(), ".")
	local major = tonumber(version[1])
	local minor = tonumber(version[2])
	local patch = tonumber(version[3])
	local build = tonumber(version[4])

	return {major, minor, patch, build}
end

local propertyBlacklist = {
	-- Mistakes
	RequestQueueSize = true, -- on ContentProvider, mistakenly marked serializable

	-- Stuff that doesn't have meaningful defaults
	ClassName = true,
	Archivable = true,
	Parent = true,
	DataCost = true,
	RobloxLocked = true,
}

local classNameBlacklist = {
	-- Creating a NetworkClient will make HTTP stop working
	NetworkClient = true,
}

local function shouldMeasureProperty(propertyDescriptor)
	if propertyBlacklist[propertyDescriptor.name] then
		return false
	end

	if propertyDescriptor.scriptability ~= "ReadWrite" then
		return false
	end

	return propertyDescriptor.isCanonical
end

--[[
	Grab a copy of an instance of the given type that should have reasonably
	default properties.
]]
local function getDefaultInstance(className)
	if classNameBlacklist[className] then
		return nil
	end

	-- Can we construct one of these from Lua?
	local ok, created = pcall(Instance.new, className)
	if ok then
		return created
	end

	-- Guess not, is it a service?
	local ok, service = pcall(game.GetService, game, className)
	if ok then
		return service
	end

	return nil
end

local function scriptabilityWithoutWrite(scriptability)
	if scriptability == "ReadWrite" then
		return "Read"
	elseif scriptability == "Write" then
		return "None"
	else
		return scriptability
	end
end

return function(postMessage)
	postMessage(HttpService:JSONEncode({
		type = "Version",
		version = getClientVersion(),
	}))

	for _, class in pairs(ReflectionClasses) do
		local instance = getDefaultInstance(class.name)

		if instance ~= nil then
			local updatedDescriptors = {}

			local currentClass = class

			while currentClass ~= nil do
				for _, propertyDescriptor in pairs(currentClass.properties) do
					if shouldMeasureProperty(propertyDescriptor) then
						local getSuccess, value = pcall(get, instance, propertyDescriptor.name)

						if getSuccess then
							local writeSuccess = pcall(set, instance, propertyDescriptor.name, value)
							local encodeSuccess, encoded = EncodedValue.encode(value, propertyDescriptor.type)

							local scriptability
							if not writeSuccess then
								scriptability = scriptabilityWithoutWrite(propertyDescriptor.scriptability)
							end

							if encodeSuccess then
								updatedDescriptors[propertyDescriptor.name] = {
									defaultValue = encoded,
									scriptability = scriptability,
								}
							else
								warn(
									"Couldn't encode property",
									propertyDescriptor.name,
									"on class",
									currentClass.name,
									encoded
								)
							end
						else
							updatedDescriptors[propertyDescriptor.name] = {
								scriptability = "None",
							}
						end
					end
				end

				currentClass = ReflectionClasses[currentClass.superclass]
			end

			if next(updatedDescriptors) ~= nil then
				postMessage(HttpService:JSONEncode({
					type = "PatchDescriptors",
					className = class.name,
					descriptors = updatedDescriptors,
				}))
			end
		end
	end

	if ERROR_AT_END then
		error("Breaking here.")
	end
end
