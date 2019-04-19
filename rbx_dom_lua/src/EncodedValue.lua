local primitiveTypes = {
	BinaryString = true,
	Bool = true,
	Content = true,
	Enum = true,
	Float32 = true,
	Float64 = true,
	Int32 = true,
	Int64 = true,
	String = true,
}

local function identity(...)
	return ...
end

local encoders = {
	Bool = identity,
	Float32 = identity,
	Float64 = identity,
	Int32 = identity,
	Int64 = identity,
	String = identity,
	BinaryString = identity,
	Content = identity,
	CFrame = function(value)
		return {value:GetComponents()}
	end,
	Color3 = function(value)
		return {value.r, value.g, value.b}
	end,
	NumberRange = function(value)
		return {value.Min, value.Max}
	end,
	Rect = function(value)
		return {value.Min.X, value.Min.Y, value.Max.X, value.Max.Y}
	end,
	UDim = function(value)
		return {value.Scale, value.Offset}
	end,
	UDim2 = function(value)
		return {value.X.Scale, value.X.Offset, value.Y.Scale, value.Y.Offset}
	end,
	Vector2 = function(value)
		return {value.X, value.Y}
	end,
	Vector2int16 = function(value)
		return {value.X, value.Y}
	end,
	Vector3 = function(value)
		return {value.X, value.Y, value.Z}
	end,
	Vector3int16 = function(value)
		return {value.X, value.Y, value.Z}
	end,
}

local directDecoders = {
	CFrame = CFrame.new,
	Color3 = Color3.new,
	Color3uint8 = Color3.fromRGB,
	NumberRange = NumberRange.new,
	Rect = Rect.new,
	UDim = UDim.new,
	UDim2 = UDim2.new,
	Vector2 = Vector2.new,
	Vector2int16 = Vector2int16.new,
	Vector3 = Vector3.new,
	Vector3int16 = Vector3int16.new,
}

local EncodedValue = {}

function EncodedValue.decode(encodedValue)
	if primitiveTypes[encodedValue.Type] then
		return true, encodedValue.Value
	end

	local constructor = directDecoders[encodedValue.Type]
	if constructor ~= nil then
		return true, constructor(unpack(encodedValue.Value))
	end

	return false
end

function EncodedValue.encode(rbxValue, reflectionType)
	if reflectionType ~= nil then
		if reflectionType.type == "data" then
			local encoder = encoders[reflectionType.name]

			if encoder ~= nil then
				return true, {
					Type = reflectionType.name,
					Value = encoder(rbxValue),
				}
			end
		elseif reflectionType.type == "enum" then
			return true, {
				Type = "Enum",
				Value = rbxValue.Value,
			}
		end
	end

	return false
end

return EncodedValue