local ReflectionDatabase = require(script.Parent.ReflectionDatabase)

local function identity(...)
	return ...
end

local canonicalProperties = {
	LocalizationTable = {
		Contents = {
			read = function(instance, key)
				return instance:GetContents()
			end,
			write = function(instance, key, value)
				return instance:SetContents(value)
			end,
		},
	},
}

local CanonicalProperty = {}

function CanonicalProperty.isScriptable(className, propertyName)
	local classDetails = ReflectionDatabase.dump.classes[className]

	if classDetails == nil then
		return true
	end

	local property = classDetails.properties[propertyName]

	if property == nil then
		return true
	end

	return not property.tags.NotScriptable
end

function CanonicalProperty.read(instance, key)
	local instanceProperties = canonicalProperties[instance.ClassName]

	if instanceProperties ~= nil then
		local methods = instanceProperties[key]

		if methods ~= nil then
			return methods.read(instance, key)
		end
	end

	return xpcall(function()
		return instance[key]
	end, identity)
end

function CanonicalProperty.write(instance, key, value)
	local instanceProperties = canonicalProperties[instance.ClassName]

	if instanceProperties ~= nil then
		local methods = instanceProperties[key]

		if methods ~= nil then
			return methods.write(instance, key, value)
		end
	end

	return xpcall(function()
		instance[key] = value
	end, identity)
end

return CanonicalProperty