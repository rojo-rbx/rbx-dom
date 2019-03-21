local RbxDom = require(game:GetService("ReplicatedStorage").RbxDom)

local baseplate = RbxDom.readInstance(workspace.Baseplate)
for key, value in pairs(baseplate) do
	print(key, value)
end