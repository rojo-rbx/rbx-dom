local RbxDom = require(game:GetService("ReplicatedStorage").RbxDom)

local CanonicalInstance = RbxDom.CanonicalInstance
local PropertySelection = RbxDom.PropertySelection

local baseplate = CanonicalInstance.readInstance(workspace.Baseplate, PropertySelection.MinimalSerializable)
for key, value in pairs(baseplate) do
	print(key, value)
end