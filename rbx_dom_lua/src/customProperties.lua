local CollectionService = game:GetService("CollectionService")
local HttpService = game:GetService("HttpService")

local Error = require(script.Parent.Error)
-- Defines how to read and write properties that aren't directly scriptable.
--
-- The reflection database refers to these as having scriptability = "Custom"
return {
	Instance = {
		Attributes = {
			read = function(instance)
				return true, instance:GetAttributes()
			end,
			write = function(instance, _, value)
				if type(value) ~= "table" then 
					local errorMessage = HttpService:JSONEncode({InstanceName = instance.Name, InputValue = value})
					return false, Error.new(Error.Kind.InvalidInput, errorMessage)
				end
				for key, _ in pairs(value) do
					if type(key) ~= "string" then
						local errorMessage = HttpService:JSONEncode({InstanceName = instance.Name, InputValue = value})
						return false, Error.new(Error.Kind.InvalidInput, errorMessage)
					end
				end
				for key, attr in pairs(value) do
					instance:SetAttribute(key, attr)
				end
				local existing = instance:GetAttributes()

				for key, attr in pairs(value) do
					instance:SetAttribute(key, attr)
				end

				for key in pairs(existing) do
					if value[key] == nil then
						instance:SetAttribute(key, nil)
					end
				end

				return true
			end,
		},
		Tags = {
			read = function(instance)
				return true, CollectionService:GetTags(instance)
			end,
			write = function(instance, _, value)
				if type(value) ~= "table" then 
					local errorMessage = HttpService:JSONEncode({InstanceName = instance.Name, InputValue = value})
					return false, Error.new(Error.Kind.InvalidInput, errorMessage)
				end
				for _, tag in ipairs(value) do
					if type(key) ~= "string" then
						local errorMessage = HttpService:JSONEncode({InstanceName = instance.Name, InputValue = value})
						return false, Error.new(Error.Kind.InvalidInput, errorMessage)
					end
				end

				local existingTags = CollectionService:GetTags(instance)

				local unseenTags = {}
				for _, tag in ipairs(existingTags) do
					unseenTags[tag] = true
				end

				for _, tag in ipairs(value) do
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
			read = function(instance)
				return true, instance:GetContents()
			end,
			write = function(instance, _, value)
				if type(value) ~= "table" then 
					local errorMessage = HttpService:JSONEncode({InstanceName = instance.Name, InputValue = value})
					return false, Error.new(Error.Kind.InvalidInput, errorMessage)
				end
				instance:SetContents(value)
				return true
			end,
		},
	},
}
