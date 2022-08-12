local database = require(script.database)
local Error = require(script.Error)
local PropertyDescriptor = require(script.PropertyDescriptor)
local EncodedValue = require(script.EncodedValue)

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
	local descriptor = findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.UnknownProperty, fullName)
	end

	return descriptor:read(instance)
end

local function writeProperty(instance, propertyName, value)
	local descriptor = findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	if descriptor == nil then
		local fullName = ("%s.%s"):format(instance.className, propertyName)

		return false, Error.new(Error.Kind.UnknownProperty, fullName)
	end

	return descriptor:write(instance, value)
end

local function findAllReadableProperties(className)
	local currentClassName = className
	local properties = {}
	repeat
		local currentClass = database.Classes[currentClassName]

		if currentClass == nil then
			return currentClass
		end

		for name , values in pairs(currentClass.Properties) do
			if values.Scriptability == "ReadWrite" or values.Scriptability == "Read" or values.Scriptability == "Custom"  then
				table.insert(properties,name)
			end
		end

		currentClassName = currentClass.Superclass
	until currentClassName == nil

	return properties
end

local function readAllReadableProperties(instance)
	if typeof(instance) ~= "Instance" then 
		local errorMessage = ("Parameter 1 instance expect an instance as input but got %s"):format(typeof(instance))
		return false, Error.new(Error.Kind.InvalidInput, errorMessage)
	end

	local read_properties = {}
	local properties = findAllReadableProperties(instance.ClassName)
	for _,property in ipairs(properties) do
		local sucess,value = readProperty(instance,property)
		if sucess then
			read_properties[property] = value	
		end
	end
	return true, read_properties
end

local function findAllDefaultProperties(className)
	local currentClassName = className
	local defaultProperties = {}
	repeat
		local currentClass = database.Classes[currentClassName]

		if currentClass == nil then
			return currentClass
		end

		for name , values in pairs(currentClass.DefaultProperties) do
			defaultProperties[name] = values
		end

		currentClassName = currentClass.Superclass
	until currentClassName == nil

	return defaultProperties
end

local function deepTableEquals(t1, t2)
	local ty1 = type(t1)
	local ty2 = type(t2)
	if ty1 ~= ty2 then return false end
	-- non-table types can be directly compared
	if ty1 ~= 'table' and ty2 ~= 'table' then return t1 == t2 end
	-- as well as tables which have the metamethod __eq
	for k1, v1 in pairs(t1) do
		local v2 = t2[k1]
		if v2 == nil or not deepTableEquals(v1, v2) then return false end
	end
	for k2, v2 in pairs(t2) do
		local v1 = t1[k2]
		if v1 == nil or not deepTableEquals(v1, v2) then return false end
	end
	return true
end

local function findAllNoneDefaultPropertiesEncoded(instance)
	if typeof(instance) ~= "Instance" then 
		local errorMessage = ("Parameter 1 instance expect an instance as input but got %s"):format(typeof(instance))
		return false, Error.new(Error.Kind.InvalidInput, errorMessage)
	end
	local noneDefaultProperties = {}
	local properties = readAllReadableProperties(instance)
	local defaultProperties = findAllDefaultProperties(instance.ClassName)
	for property ,value in pairs(properties)  do
		local sucess,enocdedPorperty = EncodedValue.encodeNaive(value)
		local defaultProperty = defaultProperties[property]
		if sucess and defaultProperty then
			if not deepTableEquals(enocdedPorperty,defaultProperty) then
				noneDefaultProperties[property] = enocdedPorperty
			end
		elseif property == "Attributes" then
			local attributes = {}
			for name, attributeValue in pairs(value) do
				local attributeSucess,enocdedattribute = EncodedValue.encodeNaive(attributeValue)
				if attributeSucess  then
					attributes[name] = enocdedattribute
				end
			end
			if not deepTableEquals(attributes,defaultProperty) then
				noneDefaultProperties[property] = attributes
			end
		end
	end
	return true, noneDefaultProperties
end
local function findAllNoneDefaultPropertiesDecoded(instance)
	if typeof(instance) ~= "Instance" then 
		local errorMessage = ("Parameter 1 instance expect an instance as input but got %s"):format(typeof(instance))
		return false, Error.new(Error.Kind.InvalidInput, errorMessage)
	end
	local noneDefaultProperties = {}
	local properties = readAllReadableProperties(instance)
	local defaultProperties = findAllDefaultProperties(instance.ClassName)
	for property ,value in pairs(properties)  do
		local sucess,enocdedPorperty = EncodedValue.encodeNaive(value)
		local defaultProperty = defaultProperties[property]
		if sucess and defaultProperty then
			if not deepTableEquals(enocdedPorperty,defaultProperty) then
				noneDefaultProperties[property] = value
			end
		elseif property == "Attributes" then
			local attributes = {}
			for name, attributeValue in pairs(value) do
				local attributeSucess,enocdedattribute = EncodedValue.encodeNaive(attributeValue)
				if attributeSucess  then
					attributes[name] = enocdedattribute
				end
			end
			if not deepTableEquals(attributes,defaultProperty) then
				noneDefaultProperties[property] = value
			end
		end
	end
	return  true, noneDefaultProperties
end

return {
	readProperty = readProperty,
	writeProperty = writeProperty,
	findCanonicalPropertyDescriptor = findCanonicalPropertyDescriptor,
	Error = Error,
	findAllReadableProperties = findAllReadableProperties,
	readAllReadableProperties = readAllReadableProperties,
	findAllDefaultProperties = findAllDefaultProperties,
	findAllNoneDefaultPropertiesEncoded = findAllNoneDefaultPropertiesEncoded,
	findAllNoneDefaultPropertiesDecoded = findAllNoneDefaultPropertiesDecoded,
	EncodedValue = EncodedValue,
}
