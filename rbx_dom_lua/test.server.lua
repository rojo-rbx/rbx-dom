local RbxDom = require(game:GetService("ReplicatedStorage").RbxDom)

local CanonicalInstance = RbxDom.CanonicalInstance
local CanonicalProperty = RbxDom.CanonicalProperty
local PropertySelection = RbxDom.PropertySelection

local baseplate = CanonicalInstance.readInstance(workspace.Baseplate, PropertySelection.MinimalSerializable)
for key, value in pairs(baseplate) do
	print(key, value)
end

print(CanonicalProperty.isScriptable("Lighting", "Technology"))