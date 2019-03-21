local ReflectionDatabase = require(script.Parent.ReflectionDatabase)
local CanonicalProperty = require(script.Parent.CanonicalProperty)

local function readInstance(instance)
	local reflectionEntry = ReflectionDatabase.dump.classes[instance.ClassName]
	local output = {}

	while reflectionEntry ~= nil do
		for key in pairs(reflectionEntry.properties) do
			local success, value = CanonicalProperty.read(instance, key)

			if success then
				output[key] = value
			else
				warn("Couldn't read:", value)
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

return readInstance