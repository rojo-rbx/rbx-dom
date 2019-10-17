//! Generates the rbx_reflection crate's reflection database.
//!
//! It uses quote to emit quasiquoted TokenStream objects. When turned into
//! strings, TokenStream objects look terrible, so we need to run rustfmt on
//! the generated code. See the `gen-reflection` script in the root for that.

use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    io::{self, Write},
};

use heck::SnakeCase;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rbx_dom_weak::RbxValue;

use crate::{
    api_dump::{Dump, DumpEnum},
    database::ReflectionDatabase,
    reflection_types::{
        RbxClassDescriptor, RbxInstanceTags, RbxPropertyDescriptor, RbxPropertyScriptability,
        RbxPropertyTags, RbxPropertyTypeDescriptor,
    },
};

pub fn emit_classes<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    let classes = generate_classes(&database.classes);
    writeln!(output, "// This file is automatically @generated.")?;

    // We have to do this as a string, or else rustfmt will leave the nasty
    // syntax that quote generates for it.
    writeln!(
        output,
        "#![allow(clippy::approx_constant, clippy::unreadable_literal)]"
    )?;

    write!(output, "{}", classes)
}

pub fn emit_enums<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    let enums = generate_enums(&database.dump);
    writeln!(output, "// This file is automatically @generated.")?;
    write!(output, "{}", enums)
}

pub fn emit_version<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    writeln!(output, "// This file is automatically @generated.")?;
    writeln!(output, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(
        output,
        "pub const RBX_VERSION_MAJOR: u32 = {};",
        database.studio_version[0]
    )?;
    writeln!(
        output,
        "pub const RBX_VERSION_MINOR: u32 = {};",
        database.studio_version[1]
    )?;
    writeln!(
        output,
        "pub const RBX_VERSION_PATCH: u32 = {};",
        database.studio_version[2]
    )?;
    writeln!(
        output,
        "pub const RBX_VERSION_BUILD: u32 = {};",
        database.studio_version[3]
    )?;

    Ok(())
}

fn get_generated_function_name(class: &str) -> Ident {
    Ident::new(
        &format!("generate_{}", class.to_snake_case()),
        Span::call_site(),
    )
}

fn generate_classes(classes: &HashMap<Cow<'static, str>, RbxClassDescriptor>) -> TokenStream {
    let mut class_names: Vec<_> = classes.keys().collect();
    class_names.sort();

    let class_functions = class_names.iter().map(|class_name| {
        let descriptor = classes.get(*class_name).unwrap();
        let function_name_token = get_generated_function_name(class_name);
        let descriptor_literal = descriptor.as_rust();

        quote! {
            fn #function_name_token() -> RbxClassDescriptor {
                return #descriptor_literal
            }
        }
    });

    let map_insertions = class_names.iter().map(|class_name| {
        let function_name_token = get_generated_function_name(class_name);
        let class_name_literal = class_name.as_rust();

        quote!(map.insert(#class_name_literal, #function_name_token()))
    });

    let len_literal = Literal::usize_unsuffixed(classes.len());

    quote! {
        use std::{
            borrow::Cow,
            collections::HashMap,
        };
        use rbx_dom_weak::{
            RbxValue,
            RbxValueType,
            NumberSequence,
            NumberSequenceKeypoint,
            ColorSequence,
            ColorSequenceKeypoint,
            Rect,
            BrickColor,
        };
        use crate::reflection_types::*;

        #(#class_functions)*

        pub fn generate_classes() -> HashMap<Cow<'static, str>, RbxClassDescriptor> {
            let mut map = HashMap::with_capacity(#len_literal);

            #(#map_insertions;)*

            map
        }
    }
}

fn generate_enums(dump: &Dump) -> TokenStream {
    let enum_len_literal = Literal::usize_unsuffixed(dump.enums.len());
    let enums = dump.enums.iter().map(emit_enum);

    quote! {
        use std::{
            borrow::Cow,
            collections::HashMap,
        };
        use crate::reflection_types::*;

        pub fn generate_enums() -> HashMap<Cow<'static, str>, RbxEnumDescriptor> {
            let mut output = HashMap::with_capacity(#enum_len_literal);
            #(#enums)*
            output
        }
    }
}

fn emit_enum(rbx_enum: &DumpEnum) -> TokenStream {
    let name_literal = Literal::string(&rbx_enum.name);
    let item_count_literal = Literal::usize_unsuffixed(rbx_enum.items.len());

    let items = rbx_enum.items.iter().map(|item| {
        let item_name = Cow::Borrowed(item.name.as_str()).as_rust();
        let item_value = Literal::u32_unsuffixed(item.value);

        quote! {
            items.insert(#item_name, #item_value);
        }
    });

    quote! {
        output.insert(Cow::Borrowed(#name_literal), RbxEnumDescriptor {
            name: Cow::Borrowed(#name_literal),
            items: {
                let mut items = HashMap::with_capacity(#item_count_literal);
                #(#items)*
                items
            },
        });
    }
}

/// Trait that describes how to turn a value into Rust code that constructs that
/// value.
trait AsRust {
    fn as_rust(&self) -> TokenStream;
}

impl AsRust for RbxClassDescriptor {
    fn as_rust(&self) -> TokenStream {
        let class_name = self.name.as_rust();
        let superclass = self.superclass.as_rust();
        let tags = self.tags.as_rust();
        let properties = self.properties.as_rust();
        let defaults = self.default_properties.as_rust();

        quote!(RbxClassDescriptor {
            name: #class_name,
            superclass: #superclass,
            tags: #tags,
            properties: #properties,
            default_properties: #defaults,
        })
    }
}

impl AsRust for RbxPropertyDescriptor {
    fn as_rust(&self) -> TokenStream {
        let member_name = self.name.as_rust();
        let resolved_type = self.value_type.as_rust();
        let tags = self.tags.as_rust();
        let is_canonical = self.is_canonical.as_rust();
        let canonical_name = self.canonical_name.as_rust();
        let serialized_name = self.serialized_name.as_rust();
        let scriptability = self.scriptability.as_rust();
        let serializes = self.serializes.as_rust();

        quote!(RbxPropertyDescriptor {
            name: #member_name,
            value_type: #resolved_type,
            tags: #tags,
            is_canonical: #is_canonical,
            canonical_name: #canonical_name,
            serialized_name: #serialized_name,
            scriptability: #scriptability,
            serializes: #serializes,
        })
    }
}

impl AsRust for RbxPropertyScriptability {
    fn as_rust(&self) -> TokenStream {
        let name = Ident::new(&format!("{:?}", self), Span::call_site());
        quote!(RbxPropertyScriptability::#name)
    }
}

impl AsRust for RbxInstanceTags {
    fn as_rust(&self) -> TokenStream {
        if self.is_empty() {
            return quote!(RbxInstanceTags::empty());
        }

        let tags = self.into_iter().map(|tag| {
            let tag_name = format!("{:?}", tag);
            let name_literal = Ident::new(&tag_name, Span::call_site());

            quote!(RbxInstanceTags::#name_literal)
        });

        quote! {
            #(#tags)|*
        }
    }
}

impl AsRust for RbxPropertyTags {
    fn as_rust(&self) -> TokenStream {
        if self.is_empty() {
            return quote!(RbxPropertyTags::empty());
        }

        let tags = self.into_iter().map(|tag| {
            let tag_name = format!("{:?}", tag);
            let name_literal = Ident::new(&tag_name, Span::call_site());

            quote!(RbxPropertyTags::#name_literal)
        });

        quote! {
            #(#tags)|*
        }
    }
}

impl AsRust for RbxValue {
    fn as_rust(&self) -> TokenStream {
        match self {
            RbxValue::String { value } => {
                let value_literal = Literal::string(value);
                quote!(RbxValue::String { value: String::from(#value_literal) })
            }
            RbxValue::BinaryString { value } => {
                let value_literal = Literal::byte_string(value);
                quote!(RbxValue::BinaryString { value: #value_literal.into() })
            }
            RbxValue::Int32 { value } => {
                let value_literal = Literal::i32_unsuffixed(*value);
                quote!(RbxValue::Int32 { value: #value_literal })
            }
            RbxValue::Int64 { value } => {
                let value_literal = Literal::i64_unsuffixed(*value);
                quote!(RbxValue::Int64 { value: #value_literal })
            }
            RbxValue::Float32 { value } => {
                let value_literal = Literal::f32_unsuffixed(*value);
                quote!(RbxValue::Float32 { value: #value_literal })
            }
            RbxValue::Float64 { value } => {
                let value_literal = Literal::f64_unsuffixed(*value);
                quote!(RbxValue::Float64 { value: #value_literal })
            }
            RbxValue::Bool { value } => {
                let value_literal = if *value {
                    Ident::new("true", Span::call_site())
                } else {
                    Ident::new("false", Span::call_site())
                };
                quote!(RbxValue::Bool { value: #value_literal })
            }
            RbxValue::Ref { .. } => quote!(RbxValue::Ref { value: None }),
            RbxValue::Vector2 { value } => {
                let x_literal = Literal::f32_unsuffixed(value[0]);
                let y_literal = Literal::f32_unsuffixed(value[1]);

                quote!(RbxValue::Vector2 { value: [#x_literal, #y_literal] })
            }
            RbxValue::Vector3 { value } => {
                let x_literal = Literal::f32_unsuffixed(value[0]);
                let y_literal = Literal::f32_unsuffixed(value[1]);
                let z_literal = Literal::f32_unsuffixed(value[2]);

                quote!(RbxValue::Vector3 { value: [#x_literal, #y_literal, #z_literal] })
            }
            RbxValue::Vector2int16 { value } => {
                let x_literal = Literal::i16_unsuffixed(value[0]);
                let y_literal = Literal::i16_unsuffixed(value[1]);

                quote!(RbxValue::Vector2int16 { value: [#x_literal, #y_literal] })
            }
            RbxValue::Vector3int16 { value } => {
                let x_literal = Literal::i16_unsuffixed(value[0]);
                let y_literal = Literal::i16_unsuffixed(value[1]);
                let z_literal = Literal::i16_unsuffixed(value[2]);

                quote!(RbxValue::Vector3int16 { value: [#x_literal, #y_literal, #z_literal] })
            }
            RbxValue::Color3 { value } => {
                let r_literal = Literal::f32_unsuffixed(value[0]);
                let g_literal = Literal::f32_unsuffixed(value[1]);
                let b_literal = Literal::f32_unsuffixed(value[2]);

                quote!(RbxValue::Color3 { value: [#r_literal, #g_literal, #b_literal] })
            }
            RbxValue::Color3uint8 { value } => {
                let r_literal = Literal::u8_unsuffixed(value[0]);
                let g_literal = Literal::u8_unsuffixed(value[1]);
                let b_literal = Literal::u8_unsuffixed(value[2]);

                quote!(RbxValue::Color3 { value: [#r_literal, #g_literal, #b_literal] })
            }
            RbxValue::CFrame { value } => {
                let literals = value.iter().cloned().map(Literal::f32_unsuffixed);

                quote!(RbxValue::CFrame {
                    value: [
                        #(#literals),*
                    ]
                })
            }
            RbxValue::Enum { value } => {
                let value_literal = Literal::u32_unsuffixed(*value);

                quote!(RbxValue::Enum { value: #value_literal })
            }
            RbxValue::PhysicalProperties { value } => {
                let value_literal = match value {
                    Some(_) => quote!(Some(PhysicalProperties)),
                    None => quote!(None),
                };

                quote!(RbxValue::PhysicalProperties { value: #value_literal })
            }
            RbxValue::UDim { value } => {
                let literal_scale = Literal::f32_unsuffixed(value.0);
                let literal_offset = Literal::i32_unsuffixed(value.1);

                quote!(RbxValue::UDim {
                    value: (#literal_scale, #literal_offset)
                })
            }
            RbxValue::UDim2 { value } => {
                let literal_x_scale = Literal::f32_unsuffixed(value.0);
                let literal_x_offset = Literal::i32_unsuffixed(value.1);
                let literal_y_scale = Literal::f32_unsuffixed(value.2);
                let literal_y_offset = Literal::i32_unsuffixed(value.3);

                quote!(RbxValue::UDim2 {
                    value: (#literal_x_scale, #literal_x_offset, #literal_y_scale, #literal_y_offset)
                })
            }
            RbxValue::Content { value } => {
                let value_literal = Literal::string(value);

                quote!(RbxValue::Content {
                    value: String::from(#value_literal),
                })
            }
            RbxValue::ColorSequence { value } => {
                let literal_keypoints = value.keypoints.iter().map(|keypoint| {
                    let time_literal = Literal::f32_unsuffixed(keypoint.time);
                    let color_r_literal = Literal::f32_unsuffixed(keypoint.color[0]);
                    let color_g_literal = Literal::f32_unsuffixed(keypoint.color[1]);
                    let color_b_literal = Literal::f32_unsuffixed(keypoint.color[2]);

                    quote!(ColorSequenceKeypoint {
                        time: #time_literal,
                        color: [#color_r_literal, #color_g_literal, #color_b_literal],
                    })
                });

                quote!(RbxValue::ColorSequence {
                    value: ColorSequence {
                        keypoints: vec![
                            #(#literal_keypoints),*
                        ],
                    },
                })
            }
            RbxValue::NumberSequence { value } => {
                let literal_keypoints = value.keypoints.iter().map(|keypoint| {
                    let time_literal = Literal::f32_unsuffixed(keypoint.time);
                    let value_literal = Literal::f32_unsuffixed(keypoint.value);
                    let envelope_literal = Literal::f32_unsuffixed(keypoint.envelope);

                    quote!(NumberSequenceKeypoint {
                        time: #time_literal,
                        value: #value_literal,
                        envelope: #envelope_literal,
                    })
                });

                quote!(RbxValue::NumberSequence {
                    value: NumberSequence {
                        keypoints: vec![
                            #(#literal_keypoints),*
                        ],
                    },
                })
            }
            RbxValue::Rect { value } => {
                let min_x_literal = Literal::f32_unsuffixed(value.min.0);
                let min_y_literal = Literal::f32_unsuffixed(value.min.1);
                let max_x_literal = Literal::f32_unsuffixed(value.max.0);
                let max_y_literal = Literal::f32_unsuffixed(value.max.1);

                quote!(RbxValue::Rect {
                    value: Rect {
                        min: (#min_x_literal, #min_y_literal),
                        max: (#max_x_literal, #max_y_literal),
                    },
                })
            }
            RbxValue::NumberRange { value } => {
                let min_literal = Literal::f32_unsuffixed(value.0);
                let max_literal = Literal::f32_unsuffixed(value.1);

                quote!(RbxValue::NumberRange {
                    value: (#min_literal, #max_literal),
                })
            }
            RbxValue::BrickColor { value } => {
                let value_literal = Literal::u16_unsuffixed(*value as u16);

                quote!(RbxValue::BrickColor {
                    value: BrickColor::from_number(#value_literal).unwrap(),
                })
            }
            _ => unimplemented!("emitting Rust type {:?}", self.get_type()),
        }
    }
}

impl<K, V> AsRust for HashMap<K, V>
where
    K: AsRust + Eq + Hash + Ord,
    V: AsRust,
{
    fn as_rust(&self) -> TokenStream {
        if self.is_empty() {
            return quote!(HashMap::new());
        }

        let len_literal = Literal::usize_unsuffixed(self.len());

        let mut keys: Vec<_> = self.keys().collect();
        keys.sort();

        let insertions = keys.iter().map(|key| {
            let value = self.get(key).unwrap();
            let key_literal = key.as_rust();
            let value_literal = value.as_rust();

            quote!(map.insert(#key_literal, #value_literal))
        });

        quote!({
            let mut map = HashMap::with_capacity(#len_literal);
            #(#insertions;)*
            map
        })
    }
}

impl AsRust for RbxPropertyTypeDescriptor {
    fn as_rust(&self) -> TokenStream {
        match self {
            RbxPropertyTypeDescriptor::Data(kind) => {
                let type_name = format!("{:?}", kind);
                let type_literal = Ident::new(&type_name, Span::call_site());
                quote!(RbxPropertyTypeDescriptor::Data(RbxValueType::#type_literal))
            }
            RbxPropertyTypeDescriptor::Enum(enum_name) => {
                let enum_literal = enum_name.as_rust();
                quote!(RbxPropertyTypeDescriptor::Enum(#enum_literal))
            }
            RbxPropertyTypeDescriptor::UnimplementedType(type_name) => {
                let type_literal = type_name.as_rust();
                quote!(RbxPropertyTypeDescriptor::UnimplementedType(#type_literal))
            }
        }
    }
}

impl AsRust for bool {
    fn as_rust(&self) -> TokenStream {
        if *self {
            quote!(true)
        } else {
            quote!(false)
        }
    }
}

impl<'a> AsRust for Cow<'a, str> {
    fn as_rust(&self) -> TokenStream {
        let literal = Literal::string(self);
        quote!(Cow::Borrowed(#literal))
    }
}

impl<T> AsRust for Option<T>
where
    T: AsRust,
{
    fn as_rust(&self) -> TokenStream {
        match self {
            Some(value) => {
                let inner_literal = value.as_rust();
                quote!(Some(#inner_literal))
            }
            None => quote!(None),
        }
    }
}
