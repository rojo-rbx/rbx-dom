local HttpService = game:GetService("HttpService")

local VERBOSE = false
local ERROR_AT_END = false

local function vwarn(message)
	if VERBOSE then
		vwarn(message)
	end
end

--[[
	Little function for use with pcall to avoid making so many closures.
]]
local function get(parent, key)
	return parent[key]
end

local function getClientVersion()
	local version = string.split(version(), ". ")
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

local function shouldMeasureProperty(propertyName, property)
	if propertyBlacklist[propertyName] then
		return false
	end

	if property.scriptability ~= "ReadWrite" then
		return false
	end

	return property.isCanonical
end

--[[
	Grab a copy of an instance of the given type that should have reasonably
	default properties.
]]
local function getDefaultInstance(className)
	if classNameBlacklist[className] then
		return false
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

local function serializeFloat(value)
	-- TODO: Figure out a better way to serialize infinity and NaN, neither of
	-- which fit into JSON.
	if value == math.huge or value == -math.huge then
		vwarn("Can't serialize infinity or negative infinity yet!")
		return 999999999 * math.sign(value)
	end

	return value
end

local typeConverters = {
	string = function(value)
		return { Type = "String", Value = value }
	end,
	number = function(value)
		-- TODO: More precise typing here somehow?

		return { Type = "Float32", Value = serializeFloat(value) }
	end,
	boolean = function(value)
		return { Type = "Bool", Value = value }
	end,
	["nil"] = function(_value)
		return { Type = "Ref", Value = nil }
	end,
	Instance = function(value)
		if value ~= nil then
			warn("Not sure how to serialize non-nil default Instance property")
		end

		return { Type = "Ref", Value = nil }
	end,
	Vector3 = function(value)
		return {
			Type = "Vector3",
			Value = {
				serializeFloat(value.X),
				serializeFloat(value.Y),
				serializeFloat(value.Z),
			}
		}
	end,
	Vector2 = function(value)
		return {
			Type = "Vector2",
			Value = {
				serializeFloat(value.X),
				serializeFloat(value.Y),
			}
		}
	end,
	Color3 = function(value)
		return {
			Type = "Color3",
			Value = {
				serializeFloat(value.r),
				serializeFloat(value.g),
				serializeFloat(value.b),
			}
		}
	end,
	CFrame = function(value)
		return {
			Type = "CFrame",
			Value = {value:components()}
		}
	end,
	EnumItem = function(value)
		return {
			Type = "Enum",
			Value = value.Value,
		}
	end,
	UDim = function(value)
		return {
			Type = "UDim",
			Value = {value.Scale, value.Offset},
		}
	end,
	UDim2 = function(value)
		return {
			Type = "UDim2",
			Value = {value.X.Scale, value.X.Offset, value.Y.Scale, value.Y.Offset},
		}
	end,
}

local function robloxValueToRojoValue(robloxValue)
	local robloxType = typeof(robloxValue)
	local converter = typeConverters[robloxType]

	if converter ~= nil then
		return true, converter(robloxValue)
	else
		return false
	end
end

return function(postMessage)
	postMessage(HttpService:JSONEncode({
		type = "Version",
		version = getClientVersion(),
	}))

	local ReflectionDatabase = {
		classes = require(script.Parent.ReflectionClasses),
	}

	for className, class in pairs(ReflectionDatabase.classes) do
		local instance = getDefaultInstance(className)

		if instance == nil then
			vwarn("Couldn't find a default enough version of instance", class.Name)
		else
			local defaultProperties = {}

			local currentClass = class

			while currentClass ~= nil do
				for propertyName, property in pairs(currentClass.properties) do
					if shouldMeasureProperty(propertyName, property) then
						local ok, value = pcall(get, instance, propertyName)

						if ok then
							local ok, rojoValue = robloxValueToRojoValue(value)

							if ok then
								defaultProperties[propertyName] = rojoValue
							else
								vwarn("Couldn't convert property", propertyName, "on class", class.Name, "to a Rojo value")
							end
						else
							vwarn("Couldn't read property", propertyName, "on class", class.Name)
						end
					end
				end

				currentClass = ReflectionDatabase.classes[currentClass.superclass]
			end

			if next(defaultProperties) ~= nil then
				postMessage(HttpService:JSONEncode({
					type = "DefaultProperties",
					className = className,
					properties = defaultProperties,
				}))
			end
		end
	end

	if ERROR_AT_END then
		error("Breaking here.")
	end
end