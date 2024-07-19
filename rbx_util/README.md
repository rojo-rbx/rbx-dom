# rbx_util

Command line tool to convert and inspect Roblox model and place files using the rbx-dom family of libraries.

Usage:

```bash
# Convert between rbxmx, rbxm, rbxl, and rbxlx
rbx-util convert input.rbxmx output.rbxm

# Debug the contents of a binary model
rbx-util view-binary output.rbxm

# Strip the specified PropertyName from all Instances of ClassName in the provided input.
# Then, write the resulting file the provided output.
rbx-util remove-prop input.rbxmx ClassName PropertyName --output output.rbxm
```
