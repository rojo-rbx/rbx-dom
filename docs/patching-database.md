# How to Fix a New Property Added by Roblox
When Roblox introduces new properties, usually tools like Rojo can use them without any additional changes. Sometimes, though, properties are added with different names, multiple serialized forms, or aren't listed at all in the reflection dump that Roblox gives us.

This document describes some common scenarios, and what work needs to happen to fix them.

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
Sometimes Roblox adds properties whose serialized names are different than their canonical names (the names exposed to users). To fix these issues, we need to introduce a _property patch_.

Property patches live in the [patches][patches] folder of the rbx-dom repository. There is roughly one YAML file per class that has changes applied to it. The `generate_reflection` tool reads these patches and uses them to generate a higher-quality reflection database for Rojo and other tools.

To fix this kind of issue, we need to introduce two different patches:
1. Add a new property descriptor for the funny name version of this property
2. Change the canonical property to indicate that it serializes with that funny name

One property that serializes with a different name is `Sound.MaxDistance`. It serializes with the name `xmlRead_MaxDistance_3`. Funky! Let's look at the two patches we need to write.

The first part of our patch file should introduce the serialized version of the property:

```yaml
Add:
  Sound:
    xmlRead_MaxDistance_3:
      AliasFor: MaxDistance
      DataType:
        Value: Float32
      Scriptability: None
```

The first two lines indicate that we're adding one or more new property descriptor for the class named `Sound`. We can have many properties here.

Next, we name the property that we're going to add. We say that it's an alias for `MaxDistance`, the canonical name for this property. It is a value type, not an enum, and the type is `Float32`. Because this property is only for serialization, it is not scriptable.

The second part of the patch we need is for adjusting the existing property, `MaxDistance`.

```yaml
Change:
  Sound:
    MaxDistance:
      Serialization:
        Type: SerializesAs
        As: xmlRead_MaxDistance_3
```

Similar to before, the first two lines indicate that we're going to change some properties on the `Sound` class. We give the property name, `MaxDistance`, and then say that we're going to change its `Serialization`. We spell out that it will serialize as `xmlRead_MaxDistance_3`.

For this property, we're done!

Here are a couple other common cases that can come up:

### The serialized property name already exists
Sometimes, we don't need to add the serialized property name as it's already in the database from the reflection dump.

One example of that is `Players.MaxPlayers`, which serializes as `MaxPlayersInternal`. For whatever reason, that property is reflected to users, and so it's already in the database.

Instead of adding a new descriptor, we can update `MaxPlayersInternal` to point to its canonical form:

```yaml
Change:
  Players:
    MaxPlayers:
      Serialization:
        Type: SerializesAs
        As: MaxPlayersInternal
    MaxPlayersInternal:
      AliasFor: MaxPlayers
```

## Roblox added a new property, but modifying it from Lua requires a special API
Sometimes a property is added that cannot be assigned directly from Lua.

First up, modify the reflection database to either add or change the property's `Scriptability` to `Custom`:

```yaml
# To add the property:
Add:
  LocalizationTable:
    # 'Contents' is the name of the field in the Roblox file formats, so it
    # makes sense to use it as the canonical name of this property.
    Contents:
      Serialization:
        Type: Serializes
      DataType:
        Value: String
      Scriptability: Custom

# To change the property:
Change:
  LocalizationTable:
    Contents:
      Scriptability: Custom
```

Next, add an entry in [`rbx_dom_lua/src/customProperties.lua`][custom-properties] and implement the `read` and `write` methods. They return whether they succeeded as their first value.

```lua
return {
	-- ...

	LocalizationTable = {
		Contents = {
			read = function(instance, key)
				return true, instance:GetContents()
			end,
			write = function(instance, key, value)
				instance:SetContents(value)
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