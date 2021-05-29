local HttpService = game:GetService("HttpService")
local Studio = settings().Studio
local Workspace = game:GetService("Workspace")

local SERVER_URL = "http://localhost:22073"

local version = string.split(version(), ".")
local major = tonumber(version[1])
local minor = tonumber(version[2])
local patch = tonumber(version[3])
local build = tonumber(version[4])

local originalRecoveryInterval = Studio["Auto-Recovery Interval (Minutes)"]
local originalRecoveryEnabled = Studio["Auto-Recovery Enabled"]

HttpService:PostAsync(SERVER_URL .. "/info", HttpService:JSONEncode({
	version = {major, minor, patch, build},
}))

Studio["Auto-Recovery Enabled"] = true
Studio["Auto-Recovery Interval (Minutes)"] = 1

-- Make a change so the auto-recovery timer starts
local part = Instance.new("Part")
part.Parent = Workspace
part:Destroy()

-- Wait until the file has been saved to set these back
HttpService:GetAsync(SERVER_URL)
Studio["Auto-Recovery Enabled"] = originalRecoveryEnabled
Studio["Auto-Recovery Interval (Minutes)"] = originalRecoveryInterval
