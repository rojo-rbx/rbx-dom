--!strict
local CollectionService = game:GetService("CollectionService")
local setAttribute = require(script.Parent.setAttribute)

type Attributes = { [string]: any }
type Unseen = { [string]: boolean }
type Tags = { string }

-- Defines how to read and write properties that aren't directly scriptable.
-- The reflection database refers to these as having scriptability = "Custom"

return {
	Instance = {
		Tags = {
			read = function(instance: Instance): (boolean, Tags)
				return true, CollectionService:GetTags(instance)
			end,

			write = function(instance: Instance, tags: Tags): boolean
				local existingTags = CollectionService:GetTags(instance)
				local unseenTags: Unseen = {}

				for _, tag in ipairs(existingTags) do
					unseenTags[tag] = true
				end

				for _, tag in ipairs(tags) do
					unseenTags[tag] = nil
					CollectionService:AddTag(instance, tag)
				end

				for tag in pairs(unseenTags) do
					CollectionService:RemoveTag(instance, tag)
				end

				return true
			end,
		},

		Attributes = {
			read = function(instance: Instance): (boolean, Attributes)
				return true, instance:GetAttributes()
			end,

			write = function (instance: Instance, attributes: Attributes): (boolean, any?)
				local existingAttributes = instance:GetAttributes()
				local unseenAttributes: Unseen = {}

				for name in pairs(existingAttributes) do
					unseenAttributes[name] = true
				end

				for name, value in pairs(attributes) do
					local ok, err = setAttribute(instance, name, value)

					if ok then
						unseenAttributes[name] = nil
					else
						return false, err
					end
				end

				for name in pairs(unseenAttributes) do
					instance:SetAttribute(name, nil)
				end

				return true
			end,
		}
	},

	LocalizationTable = {
		Contents = {
			read = function(instance: LocalizationTable): (boolean, string?)
				---@diagnostic disable-next-line: deprecated
				return true, instance:GetContents()
			end,

			write = function(instance: LocalizationTable, value: string): boolean
				---@diagnostic disable-next-line: deprecated
				instance:SetContents(value)
				return true
			end,
		},
	},
}
