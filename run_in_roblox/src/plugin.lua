while game:FindFirstChild("RUN_IN_ROBLOX_MARKER") == nil do
	game.ChildAdded:Wait()
end

local HttpService = game:GetService("HttpService")

local PORT = {{PORT}}
local SERVER_URL = ("http://localhost:%d"):format(PORT)

local function POST_MESSAGE(text)
	HttpService:PostAsync(SERVER_URL .. "/message", text)
end

HttpService:PostAsync(SERVER_URL .. "/start", "hi")

do
	{{BODY}}
end

HttpService:PostAsync(SERVER_URL .. "/finish", "hi")