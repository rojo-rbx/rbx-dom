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
	if propertyBlacklist[propertyDescriptor.Name] then
		return true
	end

	if propertyDescriptor.Scriptability ~= "ReadWrite" then
		return true
	end

	return propertyDescriptor.Kind.Canonical == nil
end

local function measure(instance, propertyDescriptor)
	local readSuccess, value = pcall(get, instance, propertyDescriptor.Name)
	local writeSuccess = pcall(set, instance, propertyDescriptor.Name, value)
	local scriptability

	if readSuccess and writeSuccess then
		scriptability = "ReadWrite"
	elseif readSuccess then
		scriptability = "Read"
	else
		scriptability = "None"
	end

	if scriptability ~= propertyDescriptor.Scriptability then
		return {
			Scriptability = scriptability,
		}
	end
end

local function getPropertyChanges(classes)
	local propertyChanges = {}

	for className, classDescriptor in pairs(classes) do
		local instance = getInstance(className)

		if instance ~= nil then
			local currentClass = className
			local changes = {}

			while currentClass ~= nil do
				for propertyName, propertyDescriptor in pairs(classDescriptor.Properties) do
					if shouldSkip(propertyDescriptor) then
						continue
					end

					changes[propertyName] = measure(instance, propertyDescriptor)
				end

				currentClass = classes[currentClass.Superclass]
			end

			if next(changes) then
				propertyChanges[className] = changes
			end
		end
	end

	return propertyChanges
end

HttpService:PostAsync(SERVER_URL, HttpService:JSONEncode({
	Version = getVersion(),
	PropertyPatches = {
		Change = getPropertyChanges(reflectionClasses),
	},
}))
