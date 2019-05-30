local CollectionService = game:GetService("CollectionService")

local ReflectionDatabase = require(script.ReflectionDatabase)
local Error = require(script.Error)

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
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.UnknownProperty, fullName)
	end

	if descriptor.scriptability == "ReadWrite" or descriptor.scriptability == "Read" then
		local success, value = xpcall(get, debug.traceback, instance, propertyName)

		if success then
			return success, value
		else
			return false, Error.new(Error.Kind.Roblox, value)
		end
	end

	if descriptor.scriptability == "Custom" then
		local interface = customProperties[descriptorClassName][descriptorName]

		return interface.read(instance, propertyName)
	end

	if descriptor.scriptability == "None" or descriptor.scriptability == "Write" then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.PropertyNotReadable, fullName)
	end
end

local function writeProperty(instance, propertyName, propertyValue)
	local descriptor, descriptorClassName, descriptorName =
		findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.UnknownProperty, fullName)
	end

	if descriptor.scriptability == "ReadWrite" or descriptor.scriptability == "Write" then
		local success, value = xpcall(set, debug.traceback, instance, propertyName, propertyValue)

		if success then
			return success
		else
			return false, Error.new(Error.Kind.Roblox, value)
		end
	end

	if descriptor.scriptability == "Custom" then
		local interface = customProperties[descriptorClassName][descriptorName]

		return interface.write(instance, propertyName, propertyValue)
	end

	if descriptor.scriptability == "None" or descriptor.scriptability == "Read" then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.PropertyNotWritable, fullName)
	end
end

return {
	readProperty = readProperty,
	writeProperty = writeProperty,
	Error = Error,
	EncodedValue = require(script.EncodedValue),
}