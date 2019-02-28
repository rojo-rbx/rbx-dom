use std::collections::HashMap;

use quote::quote;
use proc_macro2::{TokenStream, Literal, Ident, Span};
use rbx_dom_weak::RbxValue;

use crate::{
    api_dump::{Dump, DumpClass, DumpEnum, DumpClassMember, ValueType, ValueCategory},
};

pub fn generate(
    dump: &Dump,
    default_properties: &HashMap<String, HashMap<String, RbxValue>>,
) -> TokenStream {
    let class_len_literal = Literal::usize_unsuffixed(dump.classes.len());
    let enum_len_literal = Literal::usize_unsuffixed(dump.enums.len());

    let classes = dump.classes.iter().map(|class| emit_class(class, default_properties));
    let enums = dump.enums.iter().map(emit_enum);

    quote! {
        use std::collections::HashMap;
        use rbx_dom_weak::{RbxValue, RbxValueType};
        use crate::types::*;

        pub fn generate_classes() -> HashMap<&'static str, RbxInstanceClass> {
            let mut output = HashMap::with_capacity(#class_len_literal);
            #(#classes)*
            output
        }

        pub fn generate_enums() -> HashMap<&'static str, RbxEnum> {
            let mut output = HashMap::with_capacity(#enum_len_literal);
            #(#enums)*
            output
        }
    }
}

fn emit_class(
    class: &DumpClass,
    default_properties: &HashMap<String, HashMap<String, RbxValue>>,
) -> TokenStream {
    let class_name_literal = Literal::string(&class.name);

    let superclass_value = if class.superclass == "<<<ROOT>>>" {
        quote!(None)
    } else {
        let superclass_literal = Literal::string(&class.superclass);
        quote!(Some(#superclass_literal))
    };

    let defaults = emit_default_properties(&class.name, default_properties);
    let properties = emit_properties(class);

    quote! {
        output.insert(#class_name_literal, RbxInstanceClass {
            name: #class_name_literal,
            superclass: #superclass_value,
            properties: #properties,
            default_properties: #defaults,
        });
    }
}

fn emit_properties(class: &DumpClass) -> TokenStream {
    let properties: Vec<_> = class.members
        .iter()
        .filter_map(|member| match member {
            DumpClassMember::Property(property) => Some(property),
            _ => None,
        })
        .collect();

    if properties.len() == 0 {
        return quote!(HashMap::new());
    }

    let inserts = properties
        .iter()
        .map(|property| {
            let member_name = Literal::string(&property.name);
            let resolved_type = resolve_value_type(&property.value_type);

            quote! {
                properties.insert(#member_name, RbxInstanceProperty {
                    name: #member_name,
                    value_type: #resolved_type,
                });
            }
        });

    let len_literal = Literal::usize_unsuffixed(properties.len());

    quote!({
        let mut properties = HashMap::with_capacity(#len_literal);
        #(#inserts)*
        properties
    })
}

fn emit_default_properties(
    class_name: &str,
    all_defaults: &HashMap<String, HashMap<String, RbxValue>>,
) -> TokenStream {
    let defaults = match all_defaults.get(class_name) {
        Some(value) => value,
        None => return quote!(HashMap::new()),
    };

    if defaults.len() == 0 {
        return quote!(HashMap::new());
    }

    // Collect and sort keys to make output stable
    let mut keys: Vec<_> = defaults
        .keys()
        .collect();

    keys.sort();

    let inserts = keys
        .iter()
        .map(|key| {
            let value = defaults.get(key.as_str()).unwrap();

            let key_literal = Literal::string(&key);
            let value_literal = emit_value(value);

            quote!(defaults.insert(#key_literal, #value_literal);)
        });

    let len_literal = Literal::usize_unsuffixed(defaults.len());

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
            items.insert(#item_name, #item_value);
        }
    });

    quote! {
        output.insert(#name_literal, RbxEnum {
            name: #name_literal,
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
                "UDim" => quote!(RbxValueType::UDim),
                "UDim2" => quote!(RbxValueType::UDim2),

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