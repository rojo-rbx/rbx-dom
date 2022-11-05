local CollectionService = game:GetService("CollectionService")

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
				assert(type(value) == "table", ("Instance nammed: %s had wrong value: %s"):format(instance.Name,value))

				for key, attr in pairs(value) do
					instance:SetAttribute(key, attr)
				end
				local existing = instance:GetAttributes()

				for key, attr in pairs(value) do
					assert(type(key) == "string", ("Instance nammed: %s had wrong key: %s"):format(instance.Name,key))
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
				assert(type(value) == "table", ("Instance nammed: %s had wrong value: %s"):format(instance.Name,value))

				local existingTags = CollectionService:GetTags(instance)

				local unseenTags = {}
				for _, tag in ipairs(existingTags) do
					unseenTags[tag] = true
				end

				for _, tag in ipairs(value) do
					assert(type(key) == "string", ("Instance nammed: %s had wrong key: %s"):format(instance.Name,key))
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
				assert(type(value) == "table", ("Instance nammed: %s had wrong value: %s"):format(instance.Name,value))
				instance:SetContents(value)
				return true
			end,
		},
	},
}
