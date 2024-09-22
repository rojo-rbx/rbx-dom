local function isDefaultsPlace()
	-- After https://devforum.roblox.com/t/place-open-improvements-test/3086811, game.Name
	-- is no longer immediately equal to its value defined in a place file after opening a
	-- place file. Rather, game.Name is set the value defined in a place sometime after
	-- the place is opened. We need to wait for game.Name to change before validating that
	-- the place file in which this plugin is running is correct.

	-- Since if or when game.Name changes seems to be an implementation detail, we
	-- shouldn't assume game.Name will ever change. So, we'll wait for game.Name to
	-- change, or else wait out a two second timeout, whichever comes first.
	local didGameNameChange = false
	local timeElapsed = 0

	game:GetPropertyChangedSignal("Name"):Connect(function()
		didGameNameChange = true
	end)

	while not didGameNameChange and timeElapsed <= 2 do
		timeElapsed += task.wait()
	end

	if game.Name ~= "defaults-place.rbxlx" then
		return false
	end

	return true
end

if not isDefaultsPlace() then
	return
end

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
