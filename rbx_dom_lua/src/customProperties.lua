local CollectionService = game:GetService("CollectionService")
local PhysicsService = game:GetService("PhysicsService")
local HttpService = game:GetService("HttpService")

local function encodeCollisionGroup(collisionGroup)
	return string.format("%s^%i^%i", collisionGroup.name, collisionGroup.id, collisionGroup.mask)
end

local function decodeCollisionGroup(encodedCollisionGroup)
	local decoded = string.split(encodedCollisionGroup, "^")

	return {
		name = decoded[1],
		id = tonumber(decoded[2]),
		mask = tonumber(decoded[3]),
	}
end

-- Defines how to read and write properties that aren't directly scriptable.
--
-- The reflection database refers to these as having scriptability = "Custom"
return {
	Workspace = {
		CollisionGroups = {
			read = function()
				local collisionGroups = PhysicsService:GetCollisionGroups()

				-- We expect collision groups to be sorted by their IDs in increasing
				-- order. GetCollisionGroups seems to always return a list that is sorted
				-- this way, but we'll sort it here anyway to avoid relying on that.
				table.sort(collisionGroups, function(lhs, rhs)
					return lhs.id < rhs.id
				end)

				local encoded = table.create(#collisionGroups)

				for i, collisionGroup in ipairs(collisionGroups) do
					encoded[i] = encodeCollisionGroup(collisionGroup)
				end

				return true, table.concat(encoded, "\\")
			end,
			write = function(_, _, value)
				local existingCollisionGroups = PhysicsService:GetCollisionGroups()

				-- The simplest thing to do right now is remove all the existing collision
				-- groups writing the new ones. This is similar to the Tags writer's
				-- behavior of removing any unknown tags.
				for _, existingCollisionGroup in ipairs(existingCollisionGroups) do
					if existingCollisionGroup.name ~= "Default" then
						PhysicsService:RemoveCollisionGroup(existingCollisionGroup.name)
					end
				end

				local encodedCollisionGroups = string.split(value, "\\")
				local temporaryCollisionGroups = {}
				local currentCollisionGroupId = 0

				for _, encodedCollisionGroup in ipairs(encodedCollisionGroups) do
					local collisionGroup = decodeCollisionGroup(encodedCollisionGroup)

					-- The default collision group always exists and has an ID of 0, so it
					-- can be totally skipped.
					if collisionGroup.name == "Default" then
						continue
					end

					-- It's critical that we preserve the name -> ID mapping because the
					-- IDs are used to index a bitset and also to indicate parts' presence
					-- in a collision group. The encoded list of collision groups is
					-- always sorted in order of increasing ID, but there may be gaps
					-- between the IDs since collision groups may be removed. This
					-- presents a small challenge because it's not directly possible to
					-- create a collision group with a particular ID.

					while currentCollisionGroupId ~= collisionGroup.id - 1 do
						-- This path is taken when a gap is encountered. The strategy here
						-- is to backfill any gaps with temporary collision groups before
						-- creating the desired collision group (which will then have the
						-- correct ID).
						local tempName = HttpService:GenerateGUID(false)

						currentCollisionGroupId = PhysicsService:CreateCollisionGroup(tempName)
						table.insert(temporaryCollisionGroups, tempName)
					end

					currentCollisionGroupId = PhysicsService:CreateCollisionGroup(collisionGroup.name)
				end

				for _, tempName in ipairs(temporaryCollisionGroups) do
					PhysicsService:RemoveCollisionGroup(tempName)
				end

				-- Finally, we set collidabilities between each collision group.
				for _, encodedCollisionGroup in ipairs(encodedCollisionGroups) do
					local collisionGroup = decodeCollisionGroup(encodedCollisionGroup)

					for id = 0, 31 do
						local collisionGroupName = PhysicsService:GetCollisionGroupName(id)

						if collisionGroupName == nil or collisionGroupName == "" then
							continue
						end

						local isCollidable = bit32.extract(collisionGroup.mask, id) == 1 and true or false

						PhysicsService:CollisionGroupSetCollidable(collisionGroup.name, collisionGroupName, isCollidable)
					end
				end

				return true
			end,
		},
	},
	Instance = {
		Tags = {
			read = function(instance, key)
				local tagList = CollectionService:GetTags(instance)

				return true, table.concat(tagList, "\0")
			end,
			write = function(instance, key, value)
				local existingTags = CollectionService:GetTags(instance)

				local unseenTags = {}
				for _, tag in ipairs(existingTags) do
					unseenTags[tag] = true
				end

				local tagList = string.split(value, "\0")
				for _, tag in ipairs(tagList) do
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
			read = function(instance, key)
				return true, instance:GetContents()
			end,
			write = function(instance, key, value)
				instance:SetContents(value)
				return true
			end,
		},
	},
}
