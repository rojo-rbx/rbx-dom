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
	local reflectionEntry = ReflectionDatabase.dump.classes[className]
	local output = {}

	while reflectionEntry ~= nil do
		for key, propertyDetails in pairs(reflectionEntry.properties) do
			local success, value = CanonicalProperty.read(instance, key)

			if success then
				local skip = false

				if ignoreUnserializable then
					if not propertyDetails.canLoad or not propertyDetails.canSave then
						skip = true
					end
				end

				if not skip and ignoreDefaults then
					local defaultInstance = ReflectionDatabase.dump.defaults[className]
					if defaultInstance ~= nil then
						local defaultValue = defaultInstance[key]
						if equalish(value, defaultValue) then
							skip = true
						end
					end
				end

				if not skip then
					output[key] = value
				end
			else
				-- warn("Couldn't read:", value)
			end
		end

		local superclass = reflectionEntry.superclass
		if superclass == nil then
			break
		end

		reflectionEntry = ReflectionDatabase.dump.classes[superclass]
	end

	return output
end

return {
	PropertySelection = PropertySelection,
	readInstance = readInstance,
}