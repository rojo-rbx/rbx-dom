local CollectionService = game:GetService("CollectionService")

local ReflectionDatabase = require(script.ReflectionDatabase)

local customProperties = {
	Instance = {
		Tags = {
			read = function(instance, key)
				local tagList = CollectionService:GetTags(instance)

				return true, table.concat(tagList, "\0")
			end,
			write = function(instance, key, value)
				local existingTags = CollectionService:GetTags(instance)

				local unseenTags = {}
				for _, tag in ipairs(existingTags) do
					unseenTags[tag] = true
				end

				local tagList = string.split(value, "\0")
				for _, tag in ipairs(tagList) do
					unseenTags[tag] = nil
					CollectionService:AddTag(instance, tag)
				end

				for tag in pairs(unseenTags) do
					CollectionService:RemoveTag(instance, tag)
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

local function findCanonicalPropertyDescriptor(className, propertyName)
	local currentClassName = className

	repeat
		local currentClass = ReflectionDatabase.classes[currentClassName]

		local property = currentClass.properties[propertyName]
		if property ~= nil then
			if property.isCanonical then
				return property, currentClassName, propertyName
			end

			if property.canonicalName ~= nil then
				return currentClass.properties[property.canonicalName], currentClassName, property.canonicalName
			end

			return nil
		end

		currentClassName = currentClass.superclass
	until currentClassName == nil

	return nil
end

local function get(container, key)
	return container[key]
end

local function set(container, key, value)
	container[key] = value
end

local function readProperty(instance, propertyName)
	local descriptor, descriptorClassName, descriptorName =
		findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		return false, "Couldn't find descriptor for this property"
	end

	if descriptor.scriptability == "ReadWrite" or descriptor.scriptability == "Read" then
		return xpcall(get, debug.traceback, instance, propertyName)
	end

	if descriptor.scriptability == "Custom" then
		local interface = customProperties[descriptorClassName][descriptorName]

		return interface.read(instance, propertyName)
	end

	if descriptor.scriptability == "None" or descriptor.scriptability == "Write" then
		return false, ("%s.%s cannot be read."):format(instance.ClassName, propertyName)
	end
end

local function writeProperty(instance, propertyName, propertyValue)
	local descriptor, descriptorClassName, descriptorName =
		findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		return false, "Couldn't find descriptor for this property"
	end

	if descriptor.scriptability == "ReadWrite" or descriptor.scriptability == "Write" then
		return xpcall(set, debug.traceback, instance, propertyName, propertyValue)
	end

	if descriptor.scriptability == "Custom" then
		local interface = customProperties[descriptorClassName][descriptorName]

		return interface.write(instance, propertyName, propertyValue)
	end

	if descriptor.scriptability == "None" or descriptor.scriptability == "Read" then
		return false, ("%s.%s cannot be written."):format(instance.ClassName, propertyName)
	end
end

return {
	readProperty = readProperty,
	writeProperty = writeProperty,
	EncodedValue = require(script.EncodedValue),
}