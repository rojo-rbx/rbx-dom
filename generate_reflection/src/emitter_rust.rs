use std::{
    collections::HashMap,
    io::{self, Write},
};

use quote::quote;
use proc_macro2::{TokenStream, Literal, Ident, Span};
use rbx_dom_weak::RbxValue;

use crate::{
    api_dump::{Dump, DumpEnum, ValueType},
    reflection_types::{RbxInstanceClass, RbxPropertyType, RbxPropertyTags},
    database::ReflectionDatabase,
};

pub fn emit_classes<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    let classes = generate_classes(&database.classes);
    write!(output, "{}", classes)
}

pub fn emit_enums<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    let enums = generate_enums(&database.dump);
    write!(output, "{}", enums)
}

pub fn emit_version<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    writeln!(output, "pub const VERSION_MAJOR: u32 = {};", database.studio_version[0])?;
    writeln!(output, "pub const VERSION_MINOR: u32 = {};", database.studio_version[1])?;
    writeln!(output, "pub const VERSION_PATCH: u32 = {};", database.studio_version[2])?;
    writeln!(output, "pub const VERSION_BUILD: u32 = {};", database.studio_version[3])?;

    Ok(())
}

fn generate_classes(classes: &HashMap<String, RbxInstanceClass>) -> TokenStream {
    let class_len_literal = Literal::usize_unsuffixed(classes.len());

    let mut keys: Vec<_> = classes.keys().collect();
    keys.sort();

    let container_name = Ident::new("output", Span::call_site());

    let classes = keys
        .iter()
        .map(|key| generate_class(&container_name, classes.get(key.as_str()).unwrap()));

    quote! {
        use std::{
            borrow::Cow,
            collections::HashMap,
        };
        use rbx_dom_weak::{RbxValue, RbxValueType};
        use crate::reflection_types::*;

        pub fn generate_classes() -> HashMap<Cow<'static, str>, RbxInstanceClass> {
            let mut #container_name = HashMap::with_capacity(#class_len_literal);
            #(#classes)*
            #container_name
        }
    }
}

fn generate_class(container: &Ident, class: &RbxInstanceClass) -> TokenStream {
    let class_name_literal = Literal::string(&class.name);

    let superclass_value = match &class.superclass {
        None => quote!(None),
        Some(superclass) => {
            let superclass_literal = Literal::string(superclass);
            quote!(Some(Cow::Borrowed(#superclass_literal)))
        }
    };

    let tags = generate_class_tags(class);
    let properties = generate_properties(class);
    let defaults = generate_default_properties(class);

    quote! {
        #container.insert(Cow::Borrowed(#class_name_literal), RbxInstanceClass {
            name: Cow::Borrowed(#class_name_literal),
            superclass: #superclass_value,
            tags: #tags,
            properties: #properties,
            default_properties: #defaults,
        });
    }
}

fn generate_class_tags(class: &RbxInstanceClass) -> TokenStream {
    if class.tags.is_empty() {
        return quote!(RbxInstanceTags::empty());
    }

    let tags = class.tags
        .into_iter()
        .map(|tag| {
            let tag_name = format!("{:?}", tag);
            let name_literal = Ident::new(&tag_name, Span::call_site());

            quote!(RbxInstanceTags::#name_literal)
        });

    quote! {
        #(#tags)|*
    }
}

fn generate_property_tags(tags: &RbxPropertyTags) -> TokenStream {
    if tags.is_empty() {
        return quote!(RbxPropertyTags::empty());
    }

    let tags = tags
        .into_iter()
        .map(|tag| {
            let tag_name = format!("{:?}", tag);
            let name_literal = Ident::new(&tag_name, Span::call_site());

            quote!(RbxPropertyTags::#name_literal)
        });

    quote! {
        #(#tags)|*
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

        pub fn generate_enums() -> HashMap<Cow<'static, str>, RbxEnum> {
            let mut output = HashMap::with_capacity(#enum_len_literal);
            #(#enums)*
            output
        }
    }
}

fn generate_properties(class: &RbxInstanceClass) -> TokenStream {
    if class.properties.is_empty() {
        return quote!(HashMap::new());
    }

    let mut keys: Vec<_> = class.properties.keys().collect();
    keys.sort();

    let inserts = keys
        .into_iter()
        .map(|key| {
            let property = class.properties.get(key).unwrap();

            let member_name = Literal::string(&property.name);
            let resolved_type = emit_property_type(&property.value_type);
            let tags = generate_property_tags(&property.tags);

            quote! {
                properties.insert(Cow::Borrowed(#member_name), RbxInstanceProperty {
                    name: Cow::Borrowed(#member_name),
                    value_type: #resolved_type,
                    tags: #tags,
                });
            }
        });

    let len_literal = Literal::usize_unsuffixed(class.properties.len());

    quote!({
        let mut properties = HashMap::with_capacity(#len_literal);
        #(#inserts)*
        properties
    })
}

fn generate_default_properties(
    class: &RbxInstanceClass,
) -> TokenStream {
    if class.default_properties.is_empty() {
        return quote!(HashMap::new());
    }

    // Collect and sort keys to make output stable
    let mut keys: Vec<_> = class.default_properties
        .keys()
        .collect();

    keys.sort();

    let inserts = keys
        .iter()
        .map(|key| {
            let value = class.default_properties.get(*key).unwrap();

            let key_literal = Literal::string(&key);
            let value_literal = emit_value(value);

            quote!(defaults.insert(Cow::Borrowed(#key_literal), #value_literal);)
        });

    let len_literal = Literal::usize_unsuffixed(class.default_properties.len());

    quote!({
        let mut defaults = HashMap::with_capacity(#len_literal);
        #(#inserts)*
        defaults
    })
}

fn emit_enum(rbx_enum: &DumpEnum) -> TokenStream {
    let name_literal = Literal::string(&rbx_enum.name);
    let item_count_literal = Literal::usize_unsuffixed(rbx_enum.items.len());

    let items = rbx_enum.items.iter().map(|item| {
        let item_name = Literal::string(&item.name);
        let item_value = Literal::u32_unsuffixed(item.value);

        quote! {
            items.insert(Cow::Borrowed(#item_name), #item_value);
        }
    });

    quote! {
        output.insert(Cow::Borrowed(#name_literal), RbxEnum {
            name: Cow::Borrowed(#name_literal),
            items: {
                let mut items = HashMap::with_capacity(#item_count_literal);
                #(#items)*
                items
            },
        });
    }
}

fn emit_value(value: &RbxValue) -> TokenStream {
    match value {
        RbxValue::String { value } => {
            let value_literal = Literal::string(value);
            quote!(RbxValue::String { value: String::from(#value_literal) })
        },
        RbxValue::BinaryString { value } => {
            let value_literal = Literal::byte_string(value);
            quote!(RbxValue::BinaryString { value: #value_literal.into() })
        },
        RbxValue::Int32 { value } => {
            let value_literal = Literal::i32_unsuffixed(*value);
            quote!(RbxValue::Int32 { value: #value_literal })
        },
        RbxValue::Int64 { value } => {
            let value_literal = Literal::i64_unsuffixed(*value);
            quote!(RbxValue::Int64 { value: #value_literal })
        },
        RbxValue::Float32 { value } => {
            let value_literal = Literal::f32_unsuffixed(*value);
            quote!(RbxValue::Float32 { value: #value_literal })
        },
        RbxValue::Float64 { value } => {
            let value_literal = Literal::f64_unsuffixed(*value);
            quote!(RbxValue::Float64 { value: #value_literal })
        },
        RbxValue::Bool { value } => {
            let value_literal = if *value {
                Ident::new("true", Span::call_site())
            } else {
                Ident::new("false", Span::call_site())
            };
            quote!(RbxValue::Bool { value: #value_literal })
        },
        RbxValue::Ref { .. } => {
            quote!(RbxValue::Ref { value: None })
        },
        RbxValue::Vector2 { value } => {
            let x_literal = Literal::f32_unsuffixed(value[0]);
            let y_literal = Literal::f32_unsuffixed(value[1]);

            quote!(RbxValue::Vector2 { value: [#x_literal, #y_literal] })
        },
        RbxValue::Vector3 { value } => {
            let x_literal = Literal::f32_unsuffixed(value[0]);
            let y_literal = Literal::f32_unsuffixed(value[1]);
            let z_literal = Literal::f32_unsuffixed(value[2]);

            quote!(RbxValue::Vector3 { value: [#x_literal, #y_literal, #z_literal] })
        },
        RbxValue::Vector2int16 { value } => {
            let x_literal = Literal::i16_unsuffixed(value[0]);
            let y_literal = Literal::i16_unsuffixed(value[1]);

            quote!(RbxValue::Vector2int16 { value: [#x_literal, #y_literal] })
        },
        RbxValue::Vector3int16 { value } => {
            let x_literal = Literal::i16_unsuffixed(value[0]);
            let y_literal = Literal::i16_unsuffixed(value[1]);
            let z_literal = Literal::i16_unsuffixed(value[2]);

            quote!(RbxValue::Vector3int16 { value: [#x_literal, #y_literal, #z_literal] })
        },
        RbxValue::Color3 { value } => {
            let r_literal = Literal::f32_unsuffixed(value[0]);
            let g_literal = Literal::f32_unsuffixed(value[1]);
            let b_literal = Literal::f32_unsuffixed(value[2]);

            quote!(RbxValue::Color3 { value: [#r_literal, #g_literal, #b_literal] })
        },
        RbxValue::Color3uint8 { value } => {
            let r_literal = Literal::u8_unsuffixed(value[0]);
            let g_literal = Literal::u8_unsuffixed(value[1]);
            let b_literal = Literal::u8_unsuffixed(value[2]);

            quote!(RbxValue::Color3 { value: [#r_literal, #g_literal, #b_literal] })
        },
        RbxValue::CFrame { value } => {
            let literals = value.into_iter().cloned().map(Literal::f32_unsuffixed);

            quote!(RbxValue::CFrame {
                value: [
                    #(#literals),*
                ]
            })
        },
        RbxValue::Enum { value } => {
            let value_literal = Literal::u32_unsuffixed(*value);

            quote!(RbxValue::Enum { value: #value_literal })
        },
        RbxValue::PhysicalProperties { value } => {
            let value_literal = match value {
                Some(_) => quote!(Some(PhysicalProperties)),
                None => quote!(None),
            };

            quote!(RbxValue::PhysicalProperties { value: #value_literal })
        },
        RbxValue::UDim { value } => {
            let literal_scale = Literal::f32_unsuffixed(value.0);
            let literal_offset = Literal::i32_unsuffixed(value.1);

            quote!(RbxValue::UDim {
                value: (#literal_scale, #literal_offset)
            })
        },
        RbxValue::UDim2 { value } => {
            let literal_x_scale = Literal::f32_unsuffixed(value.0);
            let literal_x_offset = Literal::i32_unsuffixed(value.1);
            let literal_y_scale = Literal::f32_unsuffixed(value.2);
            let literal_y_offset = Literal::i32_unsuffixed(value.3);

            quote!(RbxValue::UDim2 {
                value: (#literal_x_scale, #literal_x_offset, #literal_y_scale, #literal_y_offset)
            })
        },
        RbxValue::Content { value } => {
            let value_literal = Literal::string(value);

            quote!(RbxValue::Content {
                value: #value_literal,
            })
        },
        _ => unimplemented!(),
    }
}

fn emit_value_type(value_type: &ValueType) -> TokenStream {
    let property_type = RbxPropertyType::from(value_type);
    emit_property_type(&property_type)
}

fn emit_property_type(property_type: &RbxPropertyType) -> TokenStream {
    match property_type {
        RbxPropertyType::Data(kind) => {
            let type_name = format!("{:?}", kind);
            let type_literal = Ident::new(&type_name, Span::call_site());
            quote!(RbxPropertyType::Data(RbxValueType::#type_literal))
        }
        RbxPropertyType::Enum(enum_name) => {
            let enum_literal = Literal::string(&enum_name);
            quote!(RbxPropertyType::Enum(Cow::Borrowed(#enum_literal)))
        }
        RbxPropertyType::UnimplementedType(type_name) => {
            let type_literal = Literal::string(&type_name);
            quote!(RbxPropertyType::UnimplementedType(Cow::Borrowed(#type_literal)))
        }
    }
}