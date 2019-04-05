local Workspace = game:GetService("Workspace")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local RbxDom = require(ReplicatedStorage.RbxDom)

local CanonicalInstance = RbxDom.CanonicalInstance
local CanonicalProperty = RbxDom.CanonicalProperty
local PropertySelection = RbxDom.PropertySelection

local baseplate = Workspace.Baseplate
local baseplateProperties = CanonicalInstance.readInstance(Workspace.Baseplate, PropertySelection.MinimalSerializable)

for key, value in pairs(baseplateProperties) do
	print(key, value)
end

local newBaseplate = Instance.new("Part")
CanonicalInstance.writeInstance(newBaseplate, baseplateProperties)

baseplate:Destroy()
newBaseplate.Parent = Workspace