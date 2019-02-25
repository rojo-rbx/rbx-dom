local HttpService = game:GetService("HttpService")

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

local blacklistedProperties = {
	-- Deprecated variants
	className = true, -- on Instance
	archivable = true, -- on Instance
	cframe = true, -- on Body*
	focus = true, -- on Camera
	CoordinateFrame = true, -- on Camera
	formFactor = true, -- on FormFactorPart
	RequestQueueSize = true, -- on ContentProvider, mistakenly marked serializable

	DataCost = true,
	RobloxLocked = true,
}

local classNameBlacklist = {
	-- Creating a NetworkClient will make HTTP stop working
	NetworkClient = true,
}

local function shouldMeasureProperty(member)
	if member.MemberType ~= "Property" then
		return false
	end

	if blacklistedProperties[member.Name] then
		return false
	end

	if not member.Serialization.CanLoad and not member.Serialization.CanSave then
		return false
	end

	return true
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

	-- Guess not, is it a service, maybe?
	local ok, service = pcall(game.GetService, game, className)

	if ok then
		return service
	end

	return nil
end

local function serializeFloat(value)
	if value == math.huge or value == -math.huge then
		warn("Can't serialize infinity or negative infinity yet!")
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
	Instance = function(_value)
		warn("Not sure how to serialize non-nil default Instance property")
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

	local dump = HttpService:JSONDecode(script.Parent.ApiDump.Value)

	local classesByName = {}
	for _, class in ipairs(dump.Classes) do
		classesByName[class.Name] = class
	end

	for _, class in ipairs(dump.Classes) do
		local instance = getDefaultInstance(class.Name)

		if instance == nil then
			warn("Couldn't find a default enough version of instance", class.Name)
		else
			local defaultProperties = {}

			local currentClass = class

			while currentClass ~= nil do
				for _, member in ipairs(currentClass.Members) do
					if shouldMeasureProperty(member) then
						local ok, value = pcall(get, instance, member.Name)

						if ok then
							local ok, rojoValue = robloxValueToRojoValue(value)

							if ok then
								defaultProperties[member.Name] = rojoValue
							else
								warn("Couldn't convert property", member.Name, "on class", class.Name, "to a Rojo value")
							end
						else
							warn("Couldn't read property", member.Name, "on class", class.Name)
						end
					end
				end

				currentClass = classesByName[currentClass.Superclass]
			end

			if next(defaultProperties) ~= nil then
				postMessage(HttpService:JSONEncode({
					type = "DefaultProperties",
					className = class.Name,
					properties = defaultProperties,
				}))
			end
		end
	end

	-- error("hold on")
end