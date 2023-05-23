# How to Fix a New Property Added by Roblox
When Roblox introduces new properties, usually tools like Rojo can use them without any additional changes. Sometimes, however, properties are added that have a different serialized name or that need to be modified with a special API in Lua.

This document describes some common scenarios and the necessary steps to fix them.

## Roblox added a new property and it serializes with the expected name and type
When Roblox introduces a new property, try saving a place or model in the XML format (rbxlx or rbxmx) and look for the property.

Here's the `BasePart.Transparency` property in Roblox's XML format:

```xml
<float name="Transparency">0</float>
```

We can see that it has the name we expect: `Transparency`. This is very good news! It also has the type we expect, `float`, which is a 32-bit floating point number.

Rojo users can specify unknown types in their projects by spelling out their types explicitly. For a `Part` with `Transparency` on Rojo 7, you could write:

```json
{
	"$className": "Part",
	"$properties": {
		"Transparency": { "Float32": 0 }
	}
}
```

This is not ideal, though. To let Rojo users write the nice, short property syntax, we'll just need to update the reflection database.

Make sure you're running the latest version of Roblox Studio, then run the `./gen-reflection` script from the [rbx-dom repo][rbx-dom]. Patching Rojo's dependencies will let you test the change and make sure it works.

## Roblox added a new property and it serializes with a _weird_ name
Sometimes a property is added with a serialized name different from its canonical name (the name exposed to users). To fix this issue, we need to introduce a _property patch_.

Property patches live in the [patches][patches] folder in the rbx-dom repository. There is roughly one YAML file per class that has changes applied to it. The `rbx_reflector` tool reads these patch files and uses them to generate an accurate reflection database for Rojo and other tools.

One property that serializes with a different name is `Sound.MaxDistance`. It serializes with the name `xmlRead_MaxDistance_3`. Funky! Let's look at the patch we need to write to fix it:

```yaml
Change:
  Sound:
    MaxDistance:
      Serialization:
        Type: SerializesAs
        As: xmlRead_MaxDistance_3
    xmlRead_MaxDistance_3:
      AliasFor: MaxDistance
```

The first two lines indicate that we're changing one or more properties for the class named `Sound`.

Next we change the `Serialization` of `MaxDistance` to serialize as `xmlRead_MaxDistance_3`.

Finally, we say that `xmlRead_MaxDistance_3` is an alias for `MaxDistance`, the canonical name of the property.

For this property, we're done!

## Roblox added a new property, but modifying it from Lua requires a special API
Sometimes a property is added that cannot be assigned directly from Lua. For example, the `Model.Scale` property:

```lua
-- This line of code throws an error: "Scale is not a valid member of Model"
model.Scale = 2
```

To fix this, first patch the property's `Scriptablity` to `Custom`:

```yaml
# To change the property:
Change:
  Model:
    Scale:
      Scriptability: Custom
```

Next, add an entry in [custom-properties] and implement the `read` and `write` methods. They return whether they succeeded as their first value.

```lua
return {
	-- ...
	Model = {
		Scale = {
			read = function(instance, key)
				return true, instance:GetScale()
			end,
			write = function(instance, key, value)
        instance:ScaleTo(value)
				return true
			end,
		},
	},
}
```

You're done! If the property is a `BinaryString` or `SharedString` when serialized, you may need to write some Rust code to transform that data into a different format.

These pull requests outline how we implemented support for Attributes in rbx-dom:

- [#166](https://github.com/rojo-rbx/rbx-dom/pull/166): Implement the attributes type in rbx_types
- [#219](https://github.com/rojo-rbx/rbx-dom/pull/219): Implement attributes in rbx_xml
- [#220](https://github.com/rojo-rbx/rbx-dom/pull/220): Implement attributes in rbx_binary
- [#228](https://github.com/rojo-rbx/rbx-dom/pull/228): Implement attributes in rbx_dom_lua
- [rojo-rbx/rojo#553](https://github.com/rojo-rbx/rojo/pull/553) and [rojo-rbx/rojo#574](https://github.com/rojo-rbx/rojo/pull/574): Support writing attributes in Rojo project files

[rbx-dom]: https://github.com/rojo-rbx/rbx-dom
[patches]: https://github.com/rojo-rbx/rbx-dom/tree/master/patches
[custom-properties]: https://github.com/rojo-rbx/rbx-dom/blob/master/rbx_dom_lua/src/customProperties.lua