//! Generates the rbx_dom_lua library's reflection database.

use std::{
    borrow::Cow,
    fmt,
    io::{self, Write},
    str,
};

use heck::CamelCase;
use lazy_static::lazy_static;
use regex::Regex;
use rbx_dom_weak::RbxValue;

use crate::{
    database::ReflectionDatabase,
    reflection_types::{RbxPropertyTypeDescriptor, RbxClassDescriptor, RbxPropertyTags, RbxPropertyScriptability},
};

lazy_static! {
    static ref LUA_IDENT: Regex = Regex::new("^[a-zA-Z_]+[a-zA-Z0-9_]*$").unwrap();
}

pub fn emit_classes<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    writeln!(output, "-- This file is automatically @generated.")?;

    let mut keys: Vec<_> = database.classes.keys().collect();
    keys.sort();

    writeln!(output, "return {{")?;
    for class_name in keys.into_iter() {
        emit_class(output, database.classes.get(class_name).unwrap())?;
    }
    writeln!(output, "}}")?;

    Ok(())
}

fn emit_class<W: Write>(output: &mut W, class: &RbxClassDescriptor) -> io::Result<()> {
    writeln!(output, "\t{} = {{", class.name)?;

    writeln!(output, "\t\tsuperclass = {},", Lua(&class.superclass))?;

    let mut keys: Vec<_> = class.properties.keys().collect();
    keys.sort();

    writeln!(output, "\t\tproperties = {{")?;
    for property_name in keys.into_iter() {
        if !LUA_IDENT.is_match(property_name) {
            continue;
        }

        let property = class.properties.get(property_name).unwrap();

        writeln!(output, "\t\t\t{} = {{", property.name)?;

        writeln!(output, "\t\t\t\ttype = {},", Lua(&property.value_type))?;
        writeln!(output, "\t\t\t\tisCanonical = {},", Lua(property.is_canonical))?;
        writeln!(output, "\t\t\t\tcanonicalName = {},", Lua(&property.canonical_name))?;
        writeln!(output, "\t\t\t\tserializedName = {},", Lua(&property.serialized_name))?;
        writeln!(output, "\t\t\t\tscriptability = {},", Lua(property.scriptability))?;
        writeln!(output, "\t\t\t\tserializes = {},", Lua(property.serializes))?;

        writeln!(output, "\t\t\t}},")?;
    }
    writeln!(output, "\t\t}},")?;

    emit_defaults(output, class)?;

    writeln!(output, "\t}},")?;
    Ok(())
}

fn emit_defaults<W: Write>(output: &mut W, class: &RbxClassDescriptor) -> io::Result<()> {
    let mut keys: Vec<_> = class.default_properties.keys().collect();
    keys.sort();

    writeln!(output, "\t\tdefaults = {{")?;
    for property_name in keys.into_iter() {
        if !LUA_IDENT.is_match(property_name) {
            continue;
        }

        let default_value = class.default_properties.get(property_name).unwrap();

        writeln!(output, "\t\t\t{} = {},", property_name, Lua(default_value))?;
    }
    writeln!(output, "\t\t}},")?;

    Ok(())
}

/// Trait that describes how to turn a type in Lua source code. It isn't aware
/// of indentation so it's only useful for things that end up on single lines.
trait AsLua {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()>;
}

impl AsLua for RbxPropertyTypeDescriptor {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        match self {
            RbxPropertyTypeDescriptor::Data(name) => {
                write!(output, "{{type = \"Data\", name = \"{:?}\"}}", name)
            }
            RbxPropertyTypeDescriptor::Enum(enum_name) => {
                write!(output, "{{type = \"Enum\", name = {}}}", Lua(enum_name))
            }
            RbxPropertyTypeDescriptor::UnimplementedType(type_name) => {
                write!(output, "{{type = \"Unimplemented\", name = {}}}", Lua(type_name))
            }
        }
    }
}

impl AsLua for RbxPropertyTags {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        write!(output, "{{")?;

        let mut is_first = true;
        for tag in self.into_iter() {
            if !is_first {
                write!(output, ", ")?;
            }

            is_first = false;

            let tag_name = format!("{:?}", tag).to_camel_case();
            write!(output, "{} = true", tag_name)?;
        }

        write!(output, "}}")
    }
}

impl AsLua for RbxPropertyScriptability {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()>  {
        format!("{:?}", self).as_lua(output)
    }
}

impl AsLua for RbxValue {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        use self::RbxValue::*;

        match self {
            BinaryString { value } => {
                output.write_all(b"\"")?;
                output.write_all(value)?;
                output.write_all(b"\"")?;
                Ok(())
            }
            Bool { value } => write!(output, "{}", *value),
            CFrame { value } => {
                write!(output,
                    "CFrame.new({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                    value[0], value[1], value[2],
                    value[3], value[4], value[5],
                    value[6], value[7], value[8],
                    value[9], value[10], value[11])
            }
            Color3 { value } => write!(output, "Color3.new({}, {}, {})", value[0], value[1], value[2]),
            Color3uint8 { value } => write!(output, "Color3.fromRGB({}, {}, {})", value[0], value[1], value[2]),
            Content { value } => write!(output, "\"{}\"", value),
            Enum { value } => write!(output, "{}", value),
            Float32 { value } => write!(output, "{}", value),
            Float64 { value } => write!(output, "{}", value),
            Int32 { value } => write!(output, "{}", value),
            Int64 { value } => write!(output, "{}", value),
            NumberRange { value } => write!(output, "NumberRange.new({}, {})", value.0, value.1),
            NumberSequence { value } => {
                write!(output, "NumberSequence.new(")?;

                for (index, keypoint) in value.keypoints.iter().enumerate() {
                    write!(output, "NumberSequenceKeypoint.new({}, {}, {})",
                        keypoint.time, keypoint.value, keypoint.envelope)?;

                    if index < value.keypoints.len() - 1 {
                        write!(output, ", ")?;
                    }
                }

                write!(output, ")")
            }
            ColorSequence { value } => {
                write!(output, "ColorSequence.new(")?;

                for (index, keypoint) in value.keypoints.iter().enumerate() {
                    write!(output, "ColorSequenceKeypoint.new({}, Color3.new({}, {}, {}))",
                        keypoint.time, keypoint.color[0], keypoint.color[1], keypoint.color[2])?;

                    if index < value.keypoints.len() - 1 {
                        write!(output, ", ")?;
                    }
                }

                write!(output, ")")
            }
            Rect { value } => {
                write!(output, "Rect.new({}, {}, {}, {})", value.min.0, value.min.1, value.max.0, value.max.1)
            }
            PhysicalProperties { value } => {
                match value {
                    Some(props) => {
                        write!(output, "PhysicalProperties.new({}, {}, {}, {}, {})",
                            props.density, props.friction, props.elasticity, props.friction_weight, props.elasticity_weight)
                    }
                    None => write!(output, "nil")
                }
            }
            Ref { value } => {
                if value.is_some() {
                    panic!("Can't serialize non-None Ref");
                }

                write!(output, "nil")
            }
            String { value } => write!(output, "\"{}\"", value),
            UDim { value } => write!(output, "UDim.new({}, {})", value.0, value.1),
            UDim2 { value } => write!(output, "UDim2.new({}, {}, {}, {})", value.0, value.1, value.2, value.3),
            Vector2 { value } => write!(output, "Vector2.new({}, {})", value[0], value[1]),
            Vector2int16 { value } => write!(output, "Vector2int16.new({}, {})", value[0], value[1]),
            Vector3 { value } => write!(output, "Vector3.new({}, {}, {})", value[0], value[1], value[2]),
            Vector3int16 { value } => write!(output, "Vector3int16.new({}, {}, {})", value[0], value[1], value[2]),
            _ => unimplemented!()
        }
    }
}

impl<'a, T> AsLua for &'a T where T: AsLua + 'a {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        (**self).as_lua(output)
    }
}

impl<'a> AsLua for Cow<'a, str> {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        write!(output, "\"{}\"", self)
    }
}

impl AsLua for str {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        // TODO: Actually escape things, this is a pretty crappy implementation.
        write!(output, "\"{}\"", self)
    }
}

impl AsLua for bool {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl<T> AsLua for Option<T> where T: AsLua {
    fn as_lua<W: Write>(&self, output: &mut W) -> io::Result<()> {
        match self {
            Some(inner) => inner.as_lua(output),
            None => write!(output, "nil"),
        }
    }
}

struct Lua<T>(T);

impl<T: AsLua> fmt::Display for Lua<T> {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Revisit this -- this feels like a GIGANTIC hack. I must not
        // fully understand io::Write vs fmt::Write.
        let mut buffer = Vec::new();
        self.0.as_lua(&mut buffer).unwrap();
        write!(output, "{}", str::from_utf8(&buffer).unwrap())
    }
}