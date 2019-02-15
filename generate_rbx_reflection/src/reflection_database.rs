use std::collections::HashMap;

use quote::quote;
use proc_macro2::{TokenStream, Literal, Ident, Span};
use rbx_dom_weak::RbxValue;

use crate::{
    api_dump::{Dump, DumpClassMember, ValueType, ValueCategory},
};

pub fn generate(
    dump: &Dump,
    default_properties: &HashMap<String, HashMap<String, RbxValue>>,
) -> TokenStream {
    let classes = dump.classes.iter().map(|class| {
        let class_name = Literal::string(&class.name);

        let superclass_value = if class.superclass == "<<<ROOT>>>" {
            quote!(None)
        } else {
            let superclass_literal = Literal::string(&class.superclass);
            quote!(Some(#superclass_literal))
        };

        let properties = class.members.iter().filter_map(|member|
            match member {
                DumpClassMember::Property { name, value_type } => {
                    let member_name = Literal::string(&name);
                    let resolved_type = resolve_value_type(value_type);

                    Some(quote! {
                        properties.insert(#member_name, RbxInstanceProperty {
                            name: #member_name,
                            value_type: #resolved_type,
                        });
                    })
                },
                _ => None,
            }
        );

        let default_properties = default_properties.get(&class.name)
            .map(|defaults| {
                defaults.iter().map(|(key, value)| {
                    let key_literal = Literal::string(&key);
                    let value_literal = emit_value(value);

                    quote!(default_properties.insert(#key_literal, #value_literal);)
                }).collect()
            })
            .unwrap_or_else(|| quote!());

        quote! {
            output.insert(#class_name, RbxInstanceClass {
                name: #class_name,
                superclass: #superclass_value,
                properties: {
                    #[allow(unused_mut)]
                    let mut properties = HashMap::new();
                    #(#properties)*
                    properties
                },
                default_properties: {
                    #[allow(unused_mut)]
                    let mut default_properties = HashMap::new();
                    #default_properties
                    default_properties
                },
            });
        }
    });

    let enums = dump.enums.iter().map(|rbx_enum| {
        let enum_name = Literal::string(&rbx_enum.name);

        let items = rbx_enum.items.iter().map(|item| {
            let item_name = Literal::string(&item.name);
            let item_value = Literal::u32_unsuffixed(item.value);

            quote! {
                items.insert(#item_name, #item_value);
            }
        });

        quote! {
            output.insert(#enum_name, RbxEnum {
                name: #enum_name,
                items: {
                    #[allow(unused_mut)]
                    let mut items = HashMap::new();
                    #(#items)*
                    items
                },
            });
        }
    });

    quote! {
        use std::collections::HashMap;
        use rbx_dom_weak::{RbxValue, RbxValueType, PhysicalProperties};
        use crate::types::*;

        pub fn generate_classes() -> HashMap<&'static str, RbxInstanceClass> {
            let mut output = HashMap::new();

            #(#classes)*

            output
        }

        pub fn generate_enums() -> HashMap<&'static str, RbxEnum> {
            let mut output = HashMap::new();

            #(#enums)*

            output
        }
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
        RbxValue::Float32 { value } => {
            let value_literal = Literal::f32_unsuffixed(*value);
            quote!(RbxValue::Float32 { value: #value_literal })
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
            let literal_0 = Literal::f32_unsuffixed(value[0]);
            let literal_1 = Literal::f32_unsuffixed(value[1]);
            let literal_2 = Literal::f32_unsuffixed(value[2]);
            let literal_3 = Literal::f32_unsuffixed(value[3]);
            let literal_4 = Literal::f32_unsuffixed(value[4]);
            let literal_5 = Literal::f32_unsuffixed(value[5]);
            let literal_6 = Literal::f32_unsuffixed(value[6]);
            let literal_7 = Literal::f32_unsuffixed(value[7]);
            let literal_8 = Literal::f32_unsuffixed(value[8]);
            let literal_9 = Literal::f32_unsuffixed(value[9]);
            let literal_10 = Literal::f32_unsuffixed(value[10]);
            let literal_11 = Literal::f32_unsuffixed(value[11]);

            quote!(RbxValue::CFrame {
                value: [
                    #literal_0, #literal_1, #literal_2,
                    #literal_3, #literal_4, #literal_5,
                    #literal_6, #literal_7, #literal_8,
                    #literal_9, #literal_10, #literal_11,
                ]
            })
        },
        RbxValue::Enum { value } => {
            let value_literal = Literal::u32_unsuffixed(*value);

            quote!(RbxValue::Enum { value: #value_literal })
        },
        RbxValue::PhysicalProperties { value } => {
            let value_literal = match value {
                Some(set) => quote!(Some(PhysicalProperties)),
                None => quote!(None),
            };

            quote!(RbxValue::PhysicalProperties { value: #value_literal })
        },
    }
}

fn resolve_value_type(value_type: &ValueType) -> TokenStream {
    let name = Literal::string(&value_type.name);

    match value_type.category {
        ValueCategory::Primitive => {
            let data_kind = match value_type.name.as_str() {
                "bool" => quote!(RbxValueType::Bool),
                "string" => quote!(RbxValueType::String),
                "int" => quote!(RbxValueType::Int32),
                "float" => quote!(RbxValueType::Float32),

                // These aren't quite right:
                "double" => quote!(RbxValueType::Float32),
                "int64" => quote!(RbxValueType::Int32),

                unknown => {
                    println!("Can't emit primitives of type {}", unknown);

                    let unknown_name = Literal::string(unknown);
                    return quote!(RbxPropertyType::UnimplementedType(#unknown_name));
                },
            };

            quote!(RbxPropertyType::Data(#data_kind))
        },
        ValueCategory::DataType => {
            let data_kind = match value_type.name.as_str() {
                "Vector3" => quote!(RbxValueType::Vector3),
                "Vector2" => quote!(RbxValueType::Vector2),
                "Color3" => quote!(RbxValueType::Color3),
                "CFrame" => quote!(RbxValueType::CFrame),
                "PhysicalProperties" => quote!(RbxValueType::PhysicalProperties),
                "BinaryString" => quote!(RbxValueType::BinaryString),

                unknown => {
                    println!("Can't emit data of type {}", unknown);

                    let unknown_name = Literal::string(unknown);
                    return quote!(RbxPropertyType::UnimplementedType(#unknown_name));
                },
            };

            quote!(RbxPropertyType::Data(#data_kind))
        },
        ValueCategory::Enum => quote!(RbxPropertyType::Enum(#name)),
        ValueCategory::Class => quote!(RbxPropertyType::Data(RbxValueType::Ref)),
    }
}