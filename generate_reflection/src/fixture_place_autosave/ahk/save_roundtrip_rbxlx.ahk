; The accompanying save_roundtrip_rbxlx.exe was compiled with Ahk2Exe, which
; comes bundled with every AutoHotKey installation.

SetTitleMatchMode, 2
if WinExist("GenerateReflectionRoundtrip.rbxlx - Roblox Studio")
{
	WinActivate
	SendInput ^s
}
else
{
	MsgBox Failed to locate a Roblox Studio window with an open GenerateReflectionRoundtrip.rbxlx.
}
