local HttpService = game:GetService("HttpService")

local SERVER_URL = "http://localhost:22073"

local reflectionClasses = HttpService:JSONDecode(HttpService:GetAsync(SERVER_URL))

local propertyBlacklist = {
	-- Mistakes
	RequestQueueSize = true, -- on ContentProvider, mistakenly marked serializable

	-- Stuff that doesn't have meaningful defaults
	Name = true,
	ClassName = true,
	Archivable = true,
	Parent = true,
	DataCost = true,
	RobloxLocked = true,
}

local classNameBlacklist = {
	-- Creating a NetworkClient will make HTTP stop working
	NetworkClient = true,
}

local function getVersion()
	local version = string.split(version(), ".")
	local major = tonumber(version[1])
	local minor = tonumber(version[2])
	local patch = tonumber(version[3])
	local build = tonumber(version[4])

	return {major, minor, patch, build}
end

local function set(instance, property, value)
	instance[property] = value
end

local function get(instance, property)
	return instance[property]
end

local function getInstance(className)
	if classNameBlacklist[className] then
		return nil
	end

	local canCreate, instance = pcall(Instance.new, className)
	if canCreate then
		return instance
	end

	local canLocate, service = pcall(game.GetService, game, className)
	if canLocate then
		return service
	end

	return nil
end

local function shouldSkip(propertyDescriptor)
	local scriptability = propertyDescriptor.Scriptability
	local isBlacklisted = propertyBlacklist[propertyDescriptor.Name]

	return
		isBlacklisted
		or scriptability == "None"
		or scriptability == "Custom"
		or propertyDescriptor.Kind.Canonical == nil
end

local function getPropertyChange(instance, propertyDescriptor)
	local propertyName = propertyDescriptor.Name
	local readSuccess, value = pcall(get, instance, propertyName)
	local writeSuccess = pcall(set, instance, propertyName, value)
	local measuredScriptability

	if readSuccess and writeSuccess then
		measuredScriptability = "ReadWrite"
	elseif readSuccess then
		measuredScriptability = "Read"
	else
		-- TODO: Are there any properties that are writable, but not
		-- readable?
		measuredScriptability = "None"
	end

	if measuredScriptability ~= propertyDescriptor.Scriptability then
		return {
			Scriptability = measuredScriptability,
		}
	else
		return nil
	end
end

local function getPropertyPatches(classes)
	local propertyChanges = {}

	for className, classDescriptor in pairs(classes) do
		local instance = getInstance(className)

		if instance == nil then
			continue
		end

		local changes = {}

		for propertyName, propertyDescriptor in pairs(classDescriptor.Properties) do
			if shouldSkip(propertyDescriptor) then
				continue
			end

			changes[propertyName] = getPropertyChange(instance, propertyDescriptor)
		end

		if next(changes) then
			propertyChanges[className] = changes
		end

		local superclassName = classDescriptor.Superclass

		while superclassName ~= nil do
			if propertyChanges[superclassName] ~= nil then
				break
			end

			local superclassDescriptor = classes[superclassName]
			local superclassChanges = {}

			for propertyName, propertyDescriptor in pairs(superclassDescriptor.Properties) do
				if shouldSkip(propertyDescriptor) then
					continue
				end

				superclassChanges[propertyName] = getPropertyChange(instance, propertyDescriptor)
			end

			if next(superclassChanges) then
				propertyChanges[superclassName] = superclassChanges
			end

			superclassName = superclassDescriptor.Superclass
		end
	end

	return {
		Change = propertyChanges,
	}
end

HttpService:PostAsync(SERVER_URL, HttpService:JSONEncode({
	Version = getVersion(),
	PropertyPatches = getPropertyPatches(reflectionClasses),
}))
