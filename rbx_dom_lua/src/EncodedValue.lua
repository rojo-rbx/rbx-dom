local EncodedValue = {}

local ALL_AXES = {"X", "Y", "Z"}
local ALL_FACES = {"Right", "Top", "Back", "Left", "Bottom", "Front"}

local PRIMITIVE_TO_VARIANT = {
	boolean = "Bool",
	string = "String",
	number = "Float64",
}

local base64 = require(script.Parent.base64)
local types

local ALL_AXES = {"X", "Y", "Z"}
local ALL_FACES = {"Right", "Top", "Back", "Left", "Bottom", "Front"}

local PRIMITIVE_TO_VARIANT = {
	boolean = "Bool",
	string = "String",
	number = "Float64",
}

local base64 = require(script.Parent.base64)
local types

local function identity(value: any): any
	return value
end

local function unpackDecoder(f: (...any) -> any): (any) -> any
	return function(value)
		return f(unpack(value))
	end
end

local function serializeFloat(value: number): any
	if value == value and math.abs(value) ~= 1/0 then
		return value
	else
		return tostring(value)
	end
end

function EncodedValue.decode(encodedValue: any): (boolean, any)
	local ty, value = next(encodedValue)
	local typeImpl = types[ty]
	
	if typeImpl == nil then
		return false, "Couldn't decode value " .. tostring(ty)
	end

	return pcall(typeImpl.fromPod, value)
end

function EncodedValue.encode(rbxValue: any, propertyType: string): (boolean, any)
	assert(propertyType ~= nil, "Property type descriptor is required")
	local typeImpl = types[propertyType]

	if typeImpl == nil then
		return false, ("Missing encoder for property type %q"):format(propertyType)
	end

	return pcall(function ()
		return {
			[propertyType] = typeImpl.toPod(rbxValue),
		}
	end)
end

types = {
	Attributes = {
		fromPod = function(pod)
			local attributes = {}

			for name, encoded in pairs(pod) do
				local ok, result = EncodedValue.decode(encoded)

				if ok then
					attributes[name] = result
					continue
				end

				local typeName = next(encoded)
				warn(("Unable to decode attribute %q (Type: %s, Error: %s)"):format(name, typeName, result))
			end

			return attributes
		end,

		toPod = function(roblox)
			local attributes = {}

			for name, value in pairs(roblox) do
				local typeName = typeof(value)

				if PRIMITIVE_TO_VARIANT[typeName] then
					typeName = PRIMITIVE_TO_VARIANT[typeName]
				end

				local ok, result = EncodedValue.encode(value, typeName)

				if ok then
					attributes[name] = result
					continue
				end

				warn(("Unable to encode attribute %q (Type: %s, Error: %s)"):format(name, typeName, result))
			end

			return attributes
		end,
	},
	
	Axes = {
		fromPod = function(pod)
			local attributes = {}

			for name, encoded in pairs(pod) do
				local ok, result = EncodedValue.decode(encoded)

				if ok then
					attributes[name] = result
					continue
				end

				local typeName = next(encoded)
				warn(("Unable to decode attribute %q (Type: %s, Error: %s)"):format(name, typeName, result))
			end

			return attributes
		end,

		toPod = function(roblox)
			local attributes = {}

			for name, value in pairs(roblox) do
				local typeName = typeof(value)

				if PRIMITIVE_TO_VARIANT[typeName] then
					typeName = PRIMITIVE_TO_VARIANT[typeName]
				end

				local ok, result = EncodedValue.encode(value, typeName)

				if ok then
					attributes[name] = result
					continue
				end

				warn(("Unable to encode attribute %q (Type: %s, Error: %s)"):format(name, typeName, result))
			end

			return attributes
		end,
	},

	Axes = {
		fromPod = function(pod): Axes
			local axes = {}

			for index, axisName in ipairs(pod) do
				axes[index] = Enum.Axis[axisName]
			end

			return Axes.new(unpack(axes))
		end,

		toPod = function(roblox: Axes)
			local pod = {}

			for _, axis in ipairs(ALL_AXES) do
				if (roblox :: any)[axis] then
					table.insert(pod, axis)
				end
			end

			return pod
		end,
	},

	BinaryString = {
		fromPod = base64.decode,
		toPod = base64.encode,
	},

	Bool = {
		fromPod = identity,
		toPod = identity,
	},

	BrickColor = {
		fromPod = function(pod: number): BrickColor
			return BrickColor.new(pod)
		end,

		toPod = function(roblox: BrickColor): number
			return roblox.Number
		end,
	},

	CFrame = {
		fromPod = function(pod)
			local pos = pod.position
			local orient = pod.orientation

			return CFrame.new(
				pos[1], pos[2], pos[3],
				orient[1][1], orient[1][2], orient[1][3],
				orient[2][1], orient[2][2], orient[2][3],
				orient[3][1], orient[3][2], orient[3][3]
			)
		end,

		toPod = function(roblox: CFrame)
			local x, y, z,
				r00, r01, r02,
				r10, r11, r12,
				r20, r21, r22 = roblox:GetComponents()

			return {
				position = {x, y, z},
				orientation = {
					{r00, r01, r02},
					{r10, r11, r12},
					{r20, r21, r22},
				},
			}
		end,
	},

	Color3 = {
		fromPod = unpackDecoder(Color3.new),

		toPod = function(roblox: Color3): {number}
			return {roblox.R, roblox.G, roblox.B}
		end,
	},

	Color3uint8 = {
		fromPod = unpackDecoder(Color3.fromRGB),

		toPod = function(roblox: Color3): {number}
			return {
				math.round(roblox.R * 255),
				math.round(roblox.G * 255),
				math.round(roblox.B * 255),
			}
		end,
	},

	ColorSequence = {
		fromPod = function(pod)
			local keypoints: {ColorSequenceKeypoint} = {}

			for index, keypoint in ipairs(pod.keypoints) do
				keypoints[index] = ColorSequenceKeypoint.new(
					keypoint.time,
					types.Color3.fromPod(keypoint.color)
				)
			end

			return ColorSequence.new(keypoints)
		end,

		toPod = function(roblox: ColorSequence)
			local keypoints = {}

			for index, keypoint in ipairs(roblox.Keypoints) do
				keypoints[index] = {
					time = keypoint.Time,
					color = types.Color3.toPod(keypoint.Value),
				}
			end

			return {
				keypoints = keypoints,
			}
		end,
	},

	Content = {
		fromPod = identity,
		toPod = identity,
	},

	Enum = {
		fromPod = identity,

		toPod = function(roblox)
			-- FIXME: More robust handling of enums
			if typeof(roblox) == "number" then
				return roblox
			else
				return roblox.Value
			end
		end,
	},

	Faces = {
		fromPod = function(pod): Faces
			local faces = {}

			for index, faceName in ipairs(pod) do
				faces[index] = Enum.NormalId[faceName]
			end

			return Faces.new(unpack(faces))
		end,

		toPod = function(roblox: Faces)
			local pod = {}

			for _, face in ipairs(ALL_FACES) do
				if (roblox :: any)[face] then
					table.insert(pod, face)
				end
			end

			return pod
		end,
	},

	Float32 = {
		fromPod = tonumber,
		toPod = serializeFloat,
	},

	Float64 = {
		fromPod = tonumber,
		toPod = serializeFloat,
	},

	Font = {
		fromPod = function(pod): Font?
			if not Font then
				-- TODO: Remove this once Font is live.
				return
			end

			local style = Enum.FontStyle[pod.Style]
			local weight = Enum.FontWeight[pod.Weight]
			return Font.new(pod.Family, weight, style)
		end,

		toPod = function(roblox: Font?)
			-- TODO: Remove this check if Font is live.
			if roblox and Font then
				return {
					Family = roblox.Family,
					Weight = roblox.Weight.Name,
					Style = roblox.Style.Name,
				}
			end
		end,
	},

	Int32 = {
		fromPod = identity,
		toPod = identity,
	},

	Int64 = {
		fromPod = identity,
		toPod = identity,
	},

	NumberRange = {
		fromPod = unpackDecoder(NumberRange.new),

		toPod = function(roblox: NumberRange)
			return {roblox.Min, roblox.Max}
		end,
	},

	NumberSequence = {
		fromPod = function(pod): NumberSequence
			local keypoints: {NumberSequenceKeypoint} = {}

			for index, keypoint in ipairs(pod.keypoints) do
				keypoints[index] = NumberSequenceKeypoint.new(
					keypoint.time,
					keypoint.value,
					keypoint.envelope
				)
			end

			return NumberSequence.new(keypoints)
		end,

		toPod = function(roblox: NumberSequence)
			local keypoints = {}

			for index, keypoint in ipairs(roblox.Keypoints) do
				keypoints[index] = {
					time = keypoint.Time,
					value = keypoint.Value,
					envelope = keypoint.Envelope,
				}
			end

			return {
				keypoints = keypoints,
			}
		end,
	},

	PhysicalProperties = {
		fromPod = function(pod): PhysicalProperties?
			if pod == "Default" then
				return nil
			else
				return PhysicalProperties.new(
					pod.density,
					pod.friction,
					pod.elasticity,
					pod.frictionWeight,
					pod.elasticityWeight
				)
			end
		end,

		toPod = function(roblox: PhysicalProperties?)
			if roblox then
				return {
					density = roblox.Density,
					friction = roblox.Friction,
					elasticity = roblox.Elasticity,
					frictionWeight = roblox.FrictionWeight,
					elasticityWeight = roblox.ElasticityWeight,
				}
			else
				return "Default"
			end
		end,
	},

	Ray = {
		fromPod = function(pod): Ray
			return Ray.new(
				types.Vector3.fromPod(pod.origin),
				types.Vector3.fromPod(pod.direction)
			)
		end,

		toPod = function(roblox: Ray)
			return {
				origin = types.Vector3.toPod(roblox.Origin),
				direction = types.Vector3.toPod(roblox.Direction),
			}
		end,
	},

	Rect = {
		fromPod = function(pod): Rect
			return Rect.new(
				types.Vector2.fromPod(pod[1]),
				types.Vector2.fromPod(pod[2])
			)
		end,

		toPod = function(roblox: Rect)
			return {
				types.Vector2.toPod(roblox.Min),
				types.Vector2.toPod(roblox.Max),
			}
		end,
	},

	Ref = {
		fromPod = function(_pod)
			error("Ref cannot be decoded on its own")
		end,

		toPod = function(_roblox)
			error("Ref can not be encoded on its own")
		end,
	},

	Region3 = {
		fromPod = function(_pod)
			error("Region3 is not implemented")
		end,

		toPod = function(_roblox)
			error("Region3 is not implemented")
		end,
	},

	Region3int16 = {
		fromPod = function(pod): Region3int16
			return Region3int16.new(
				types.Vector3int16.fromPod(pod[1]),
				types.Vector3int16.fromPod(pod[2])
			)
		end,

		toPod = function(roblox: Region3int16)
			return {
				types.Vector3int16.toPod(roblox.Min),
				types.Vector3int16.toPod(roblox.Max),
			}
		end,
	},

	SharedString = {
		fromPod = function(_pod)
			error("SharedString is not supported")
		end,

		toPod = function(_roblox)
			error("SharedString is not supported")
		end,
	},

	String = {
		fromPod = identity,
		toPod = identity,
	},

	UDim = {
		fromPod = unpackDecoder(UDim.new),

		toPod = function(roblox: UDim)
			return {roblox.Scale, roblox.Offset}
		end,
	},

	UDim2 = {
		fromPod = function(pod): UDim2
			return UDim2.new(
				types.UDim.fromPod(pod[1]),
				types.UDim.fromPod(pod[2])
			)
		end,

		toPod = function(roblox: UDim2)
			return {
				types.UDim.toPod(roblox.X),
				types.UDim.toPod(roblox.Y),
			}
		end,
	},

	Tags = {
		fromPod = identity,
		toPod = identity,
	},

	Vector2 = {
		fromPod = unpackDecoder(Vector2.new),

		toPod = function(roblox: Vector2)
			return {
				serializeFloat(roblox.X),
				serializeFloat(roblox.Y),
			}
		end,
	},

	Vector2int16 = {
		fromPod = unpackDecoder(Vector2int16.new),

		toPod = function(roblox: Vector2int16)
			return {roblox.X, roblox.Y}
		end,
	},

	Vector3 = {
		fromPod = unpackDecoder(Vector3.new),

		toPod = function(roblox: Vector3)
			return {
				serializeFloat(roblox.X),
				serializeFloat(roblox.Y),
				serializeFloat(roblox.Z),
			}
		end,
	},

	Vector3int16 = {
		fromPod = unpackDecoder(Vector3int16.new),

		toPod = function(roblox: Vector3int16)
			return {roblox.X, roblox.Y, roblox.Z}
		end,
	},
}

return EncodedValue
