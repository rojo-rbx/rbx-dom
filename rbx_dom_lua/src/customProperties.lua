-- Defines how to read and write properties that aren't directly scriptable.
-- The reflection database refers to these as having scriptability = "Custom"

local CollectionService = game:GetService("CollectionService")
local InsertService = game:GetService("InsertService")
local MESH_DISPATCH: { [MeshPart]: thread } = {}

return {
	Instance = {
		Attributes = {
			read = function(instance)
				return true, instance:GetAttributes()
			end,
			write = function(instance, _, value)
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
			read = function(instance, key)
				return true, instance:GetContents()
			end,
			write = function(instance, key, value)
				instance:SetContents(value)
				return true
			end,
		},
	},
	MeshPart = {
		MeshID = {
			read = function(instance)
				return true, instance.MeshId
			end,
			write = function(meshPart: MeshPart, _, meshId: string)
				if meshPart.MeshId == meshId then
					return
				end

				local dispatch: thread = MESH_DISPATCH[meshPart]

				if dispatch then
					task.cancel(dispatch)
				end

				dispatch = task.defer(function()
					local applied = false
					local lastErr: string
					local retries = 0

					while not applied do
						local success: boolean, response: MeshPart | string = pcall(function()
							return InsertService:CreateMeshPartAsync(meshId, meshPart.CollisionFidelity, meshPart.RenderFidelity)
						end)

						if success and typeof(response) == "Instance" then
							-- If the properties of what we're applying differ from what the MeshPart
							-- currently wants, try again without increasing the retry counter.

							if response.CollisionFidelity ~= meshPart.CollisionFidelity then
								continue
							end

							if response.RenderFidelity ~= meshPart.RenderFidelity then
								continue
							end

							meshPart:ApplyMesh(response)
							applied = true

							break
						elseif typeof(response) == "string" then
							lastErr = response

							if lastErr:find("invalid mesh asset") then
								break
							end

							retries += 1

							if retries < 5 then
								task.wait(retries / 2)
							else
								break
							end
						end

						error("Unexpected response from CreateMeshPartAsync!", 2)
					end

					if not applied then
						warn("Error creating MeshPart:", lastErr)
					end

					if MESH_DISPATCH[meshPart] == dispatch then
						MESH_DISPATCH[meshPart] = nil
					end
				end)

				MESH_DISPATCH[meshPart] = dispatch
				return true
			end,
		},
	}
}
