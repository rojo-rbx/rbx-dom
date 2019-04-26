local ReflectionDatabase = require(script.Parent.ReflectionDatabase)
local CanonicalProperty = require(script.Parent.CanonicalProperty)

local PropertySelection = {
	All = {
		ignoreDefaults = false,
		ignoreUnserializable = false,
	},
	Minimal = {
		ignoreDefaults = true,
		ignoreUnserializable = false,
	},
	MinimalSerializable = {
		ignoreDefaults = true,
		ignoreUnserializable = true,
	},
	Serializable = {
		ignoreDefaults = false,
		ignoreUnserializable = true,
	},
}

local function equalish(fromInstance, fromDatabase)
	local typeA = typeof(fromInstance)
	local typeB = typeof(fromDatabase)

	if typeA == "EnumItem" and typeB == "number" then
		return fromInstance.Value == fromDatabase
	else
		return fromInstance == fromDatabase
	end
end

local function readInstance(instance, selectionMode)
	if selectionMode == nil then
		selectionMode = PropertySelection.All
	end

	local ignoreDefaults = selectionMode.ignoreDefaults
	local ignoreUnserializable = selectionMode.ignoreUnserializable

	local className = instance.ClassName
	local reflectionEntry = ReflectionDatabase.classes[className]
	local initialReflectionEntry = reflectionEntry
	local output = {}

	while reflectionEntry ~= nil do
		for key, propertyDetails in pairs(reflectionEntry.properties) do
			if propertyDetails.isCanonical then
				local success, value = CanonicalProperty.read(instance, key)

				if success then
					local skip = false

					if ignoreUnserializable then
						skip = not propertyDetails.serializes
					end

					if not skip and ignoreDefaults then
						local defaultValue = initialReflectionEntry.defaults[key]
						if defaultValue ~= nil and equalish(value, defaultValue) then
							skip = true
						end
					end

					if not skip then
						output[key] = value
					end
				end
			end
		end

		local superclass = reflectionEntry.superclass
		if superclass == nil then
			break
		end

		reflectionEntry = ReflectionDatabase.classes[superclass]
	end

	return output
end

local function writeInstance(instance, properties)
	for key, value in pairs(properties) do
		CanonicalProperty.write(instance, key, value)
	end
end

local function writeInstanceOne(instance, propertyName, propertyValue)
	writeInstance(instance, {
		[propertyName] = propertyValue,
	})
end

return {
	PropertySelection = PropertySelection,
	readInstance = readInstance,
	writeInstance = writeInstance,
	writeInstanceOne = writeInstanceOne,
}