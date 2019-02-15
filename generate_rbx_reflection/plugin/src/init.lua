local HttpService = game:GetService("HttpService")

return function(POST_MESSAGE)
	local version = string.split(version(), ". ")
	local major = tonumber(version[1])
	local minor = tonumber(version[2])
	local patch = tonumber(version[3])
	local build = tonumber(version[4])

	POST_MESSAGE(HttpService:JSONEncode({
		type = "Version",
		version = {major, minor, patch, build},
	}))
end