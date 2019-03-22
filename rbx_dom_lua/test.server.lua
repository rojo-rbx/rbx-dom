local RbxDom = require(game:GetService("ReplicatedStorage").RbxDom)
local CanonicalInstance = RbxDom.CanonicalInstance

local baseplate = CanonicalInstance.readInstance(workspace.Baseplate, CanonicalInstance.PropertySelection.MinimalSerializable)
for key, value in pairs(baseplate) do
	print(key, value)
end