# How to Fix a New Property Added by Roblox
When Roblox introduces new properties, usually tools like Rojo can use them without any additional changes. Sometimes though, properties are added with different names, multiple serialized forms, need to be migrated to a new property, or aren't listed at all in the reflection dump that Roblox gives us.

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

## Roblox added a new property, but it's a migration from an existing property, and the existing property no longer loads
Sometimes Roblox migrates an existing property whose type is too constrained to a new property with a more flexible type.

This can cause problems when binary files containing the old property and binary files containing the new property are placed together in the same DOM, then serialized with `rbx_binary`. In the Roblox binary format, all instances of a class must define the same properties, so for instances from old files (where the new property is missing), `rbx_binary` simply writes the new property with a default value to uphold the invariant. This can result in weird behavior like old text UI all having the Arial font, because the default value of a new property took priority.

To fix this, we need to write a migration (in Rust) and apply it is as a patch (using database patch files), so that the old property is translated to the new property on deserialization.

Note that migration does *not* change the old files by itself - the process occurs only during deserialization, is purely in-memory, and will not overwrite old files with new versions.

First, add your migration to the `MigrationOperation` enum in [`rbx_reflection/src/migration.rs`][migrations]. The migration should be named after the properties it's migrating. For example, migrating from `Font` to `FontFace` would be named `FontToFontFace`.

Next, add code to convert from the old property's type to the new property's type. This code should be a new match arm in the `PropertyMigration::perform` method in [`rbx_reflection/src/migration.rs`][migrations].

Finally, add a patch in the [patches](patches) folder. This patch should change the old property's serialization type to `Migrate`, specifying the new property name and the migration name.

For example, the patch for fonts looks like:
```yaml
Change:
  TextLabel:
    Font: # Property we're migrating *from*
      Serialization:
        Type: Migrate
        To: FontFace # Name of the property we're migrating to
        Migration: FontToFontFace # Name of the migration operation that should convert the old property value to the new one
```

If this property is present on multiple classes, you may need to specify the Serialization change for multiple properties on multiple classes. For example, the `Font` property is present on `TextLabel`, `TextButton`, `TextBox` without being derived from a superclass, so the real patch is approximately 3 times as long since it needs to be applied to each class.

## Roblox added a new property, but modifying it from Lua requires a special API
Sometimes a property is added that cannot be assigned directly from Lua. For example, the `Model.Scale` property:

```lua
-- This line of code throws an error: "Scale is not a valid member of Model"
model.Scale = 2
```

To fix this, first modify the reflection database to either add or change the property's `Scriptability` to `Custom`:

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
[migrations]: https://github.com/rojo-rbx/rbx-dom/blob/master/rbx_reflection/src/migration.rs
