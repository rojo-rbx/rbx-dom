return function()
	local PhysicsService = game:GetService("PhysicsService")

	local customProperties = require(script.Parent.customProperties)

	local function clearCollisionGroups()
		for _, collisionGroup in ipairs(PhysicsService:GetCollisionGroups()) do
			if collisionGroup.name ~= "Default" then
				PhysicsService:RemoveCollisionGroup(collisionGroup.name)
			end
		end
	end

	local function doesCollisionGroupExist(name)
		return pcall(PhysicsService.GetCollisionGroupId, PhysicsService, name)
	end

	describe("CollisionGroups", function()
		describe("write", function()
			local writeCollisionGroups = customProperties.Workspace.CollisionGroups.write

			it("should write a list of collision groups when they have sequential IDs", function()
				clearCollisionGroups()

				writeCollisionGroups(nil, nil, "Default^0^-1\\Time^1^-1\\Doesn't^2^-1\\Exist^3^-1")

				expect(PhysicsService:GetCollisionGroupId("Time")).to.equal(1)
				expect(PhysicsService:GetCollisionGroupId("Doesn't")).to.equal(2)
				expect(PhysicsService:GetCollisionGroupId("Exist")).to.equal(3)
			end)

			it("should write a list of collision groups with gaps in between their IDs", function()
				clearCollisionGroups()

				writeCollisionGroups(nil, nil, "Default^0^-1\\Clocks^2^-1\\Do^5^-1\\Exist^9^-1")

				expect(PhysicsService:GetCollisionGroupId("Clocks")).to.equal(2)
				expect(PhysicsService:GetCollisionGroupId("Do")).to.equal(5)
				expect(PhysicsService:GetCollisionGroupId("Exist")).to.equal(9)

				-- The developer hub documentation says that GetCollisionGroupName returns
				-- nil for a "group that has not been named." If the group was named and
				-- then removed it seems to return the empty string :D
				expect(PhysicsService:GetCollisionGroupName(1)).to.equal("")
				expect(PhysicsService:GetCollisionGroupName(3)).to.equal("")
				expect(PhysicsService:GetCollisionGroupName(4)).to.equal("")
				expect(PhysicsService:GetCollisionGroupName(6)).to.equal("")
				expect(PhysicsService:GetCollisionGroupName(7)).to.equal("")
				expect(PhysicsService:GetCollisionGroupName(8)).to.equal("")
			end)

			it("should work when there were collision groups defined previously", function()
				clearCollisionGroups()

				PhysicsService:CreateCollisionGroup("A")
				PhysicsService:CreateCollisionGroup("B")
				PhysicsService:CreateCollisionGroup("C")
				PhysicsService:CreateCollisionGroup("D")
				PhysicsService:CreateCollisionGroup("E")

				-- And for good measure we'll also remove some
				PhysicsService:RemoveCollisionGroup("B")
				PhysicsService:RemoveCollisionGroup("D")

				writeCollisionGroups(nil, nil, "Default^0^-1\\Clocks^2^-1\\Do^5^-1\\Exist^9^-1")

				expect(PhysicsService:GetCollisionGroupId("Clocks")).to.equal(2)
				expect(PhysicsService:GetCollisionGroupId("Do")).to.equal(5)
				expect(PhysicsService:GetCollisionGroupId("Exist")).to.equal(9)
			end)

			it("should preserve the collidability of collision groups", function()
				clearCollisionGroups()

				writeCollisionGroups(nil, nil, "Default^0^-3\\A^2^-262\\B^5^-321\\C^6^-33\\D^8^-37")

				expect(PhysicsService:CollisionGroupsAreCollidable("A", "A")).to.equal(false)
				expect(PhysicsService:CollisionGroupsAreCollidable("A", "Default")).to.equal(false)

				expect(PhysicsService:CollisionGroupsAreCollidable("C", "B")).to.equal(false)

				expect(PhysicsService:CollisionGroupsAreCollidable("D", "A")).to.equal(false)
				expect(PhysicsService:CollisionGroupsAreCollidable("D", "B")).to.equal(false)
			end)
		end)

		describe("read", function()
			local readCollisionGroups = customProperties.Workspace.CollisionGroups.read

			it("should read all collision groups", function()
				clearCollisionGroups()

				PhysicsService:CreateCollisionGroup("A")
				PhysicsService:CreateCollisionGroup("B")
				PhysicsService:CreateCollisionGroup("C")
				PhysicsService:CreateCollisionGroup("D")

				local _, encoded = readCollisionGroups()

				expect(encoded).to.equal("Default^0^-1\\A^1^-1\\B^2^-1\\C^3^-1\\D^4^-1")
			end)
		end)
	end)
end
