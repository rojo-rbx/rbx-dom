local base64 = require(script.Parent.base64)

local function identity(...)
	return ...
end

local function unpackDecoder(f)
	return function(value)
		return f(unpack(value))
	end
end

local encoders = {
	Bool = identity,
	Content = identity,
	Float32 = identity,
	Float64 = identity,
	Int32 = identity,
	Int64 = identity,
	String = identity,

	BinaryString = base64.encode,
	SharedString = base64.encode,

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

local decoders = {
	Bool = identity,
	Content = identity,
	Enum = identity,
	Float32 = identity,
	Float64 = identity,
	Int32 = identity,
	Int64 = identity,
	String = identity,

	BinaryString = base64.decode,
	SharedString = base64.decode,

	CFrame = unpackDecoder(CFrame.new),
	Color3 = unpackDecoder(Color3.new),
	Color3uint8 = unpackDecoder(Color3.fromRGB),
	NumberRange = unpackDecoder(NumberRange.new),
	Rect = unpackDecoder(Rect.new),
	UDim = unpackDecoder(UDim.new),
	UDim2 = unpackDecoder(UDim2.new),
	Vector2 = unpackDecoder(Vector2.new),
	Vector2int16 = unpackDecoder(Vector2int16.new),
	Vector3 = unpackDecoder(Vector3.new),
	Vector3int16 = unpackDecoder(Vector3int16.new),

	PhysicalProperties = function(properties)
		if properties == nil then
			return nil
		else
			return PhysicalProperties.new(
				properties.density,
				properties.friction,
				properties.elasticity,
				properties.frictionWeight,
				properties.elasticityWeight
			)
		end
	end,
}

local EncodedValue = {}

function EncodedValue.decode(encodedValue)
	local decoder = decoders[encodedValue.Type]
	if decoder ~= nil then
		return true, decoder(encodedValue.Value)
	end

	return false, "Couldn't decode value " .. tostring(encodedValue.Type)
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

	return false, "Couldn't encode value"
end

return EncodedValue