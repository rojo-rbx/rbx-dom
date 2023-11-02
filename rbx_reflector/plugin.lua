local HttpService = game:GetService("HttpService")

local SERVER_URL = "http://localhost:22073"

local version = string.split(version(), ".")
local major = tonumber(version[1])
local minor = tonumber(version[2])
local patch = tonumber(version[3])
local build = tonumber(version[4])

HttpService:PostAsync(
	SERVER_URL,
	HttpService:JSONEncode({
		version = { major, minor, patch, build },
	})
)
