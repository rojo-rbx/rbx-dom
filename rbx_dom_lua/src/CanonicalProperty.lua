local CollectionService = game:GetService("CollectionService")

local ReflectionDatabase = require(script.Parent.ReflectionDatabase)

local function identity(...)
	return ...
end

local canonicalProperties = {
	-- TODO: Terrain
	Instance = {
		Tags = {
			read = function(instance, key)
				local tagList = CollectionService:GetTags(instance)

				return true, table.concat(tagList, "\0")
			end,
			write = function(instance, key, value)
				local tagList = string.split(value, "\0")

				for _, tag in ipairs(tagList) do
					CollectionService:AddTag(instance, tag)
				end

				return true
			end,
		},
	},
	LocalizationTable = {
		Contents = {
			read = function(instance, key)
				return true, instance:GetContents()
			end,
			write = function(instance, key, value)
				instance:SetContents(value)
				return true
			end,
		},
	},
}

local function findCanonicalGetterSetter(className, propertyName)
	repeat
		local instanceProperties = canonicalProperties[className]

		if instanceProperties ~= nil then
			local property = instanceProperties[propertyName]

			if property ~= nil then
				return property
			end
		end

		local classDetails = ReflectionDatabase.dump.classes[className]

		if classDetails == nil then
			return nil
		end

		className = classDetails.superclass
	until className == nil
end

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

function CanonicalProperty.read(instance, propertyName)
	local property = findCanonicalGetterSetter(instance.ClassName, propertyName)

	if property ~= nil then
		return property.read(instance, propertyName)
	end

	return xpcall(function()
		return instance[propertyName]
	end, identity)
end

function CanonicalProperty.write(instance, propertyName, value)
	local property = findCanonicalGetterSetter(instance.ClassName, propertyName)

	if property ~= nil then
		return property.write(instance, propertyName, value)
	end

	return xpcall(function()
		instance[propertyName] = value
	end, identity)
end

return CanonicalProperty