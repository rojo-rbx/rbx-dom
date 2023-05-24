local HttpService = game:GetService("HttpService")

local SERVER_URL = "http://localhost:22073"

HttpService:PostAsync(SERVER_URL, "")
