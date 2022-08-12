local database = require(script.database)
local Error = require(script.Error)
local PropertyDescriptor = require(script.PropertyDescriptor)

local function findCanonicalPropertyDescriptor(className, propertyName)
	local currentClassName = className

	repeat
		local currentClass = database.Classes[currentClassName]

		if currentClass == nil then
			return currentClass
		end

		local propertyData = currentClass.Properties[propertyName]
		if propertyData ~= nil then
			local canonicalData = propertyData.Kind.Canonical
			if canonicalData ~= nil then
				return PropertyDescriptor.fromRaw(propertyData, currentClassName, propertyName)
			end

			local aliasData = propertyData.Kind.Alias
			if aliasData ~= nil then
				return PropertyDescriptor.fromRaw(
					currentClass.Properties[aliasData.AliasFor],
					currentClassName,
					aliasData.AliasFor)
			end

			return nil
		end

		currentClassName = currentClass.Superclass
	until currentClassName == nil

	return nil
end

local function readProperty(instance, propertyName)
	if typeof(instance) ~= "Instance" then 
		local errorMessage = ("Parameter 1 instance expect an instance as input but got %s"):format(typeof(instance))
		return false, Error.new(Error.Kind.InvalidInput, errorMessage)
	end
	local descriptor = findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.UnknownProperty, fullName)
	end

	return descriptor:read(instance)
end

local function writeProperty(instance, propertyName, value)
	if typeof(instance) ~= "Instance" then 
		local errorMessage = ("Parameter 1 instance expect an instance as input but got %s"):format(typeof(instance))
		return false, Error.new(Error.Kind.InvalidInput, errorMessage)
	end
	if type(value) ~= "string" then 
		local errorMessage = ("Parameter 2 propertyName expect an string as input but got %s"):format(typeof(instance))
		return false, Error.new(Error.Kind.InvalidInput, errorMessage)
	end
	local descriptor = findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.UnknownProperty, fullName)
	end

	return descriptor:write(instance, value)
end

return {
	readProperty = readProperty,
	writeProperty = writeProperty,
	findCanonicalPropertyDescriptor = findCanonicalPropertyDescriptor,
	Error = Error,
	EncodedValue = require(script.EncodedValue),
}
